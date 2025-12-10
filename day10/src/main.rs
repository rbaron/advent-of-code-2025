use cached::proc_macro::cached;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use std::collections::{HashSet, VecDeque};
use z3::{Config, Context, Solver, ast::Int};

#[derive(Debug)]
struct Mac {
    lights: u16,
    btns: Vec<u16>,
    joltages: Vec<u16>,
}

fn ns_to_bitmask(ns: &[u16]) -> u16 {
    ns.iter().fold(0u16, |acc, v| acc | 1u16 << v)
}

#[cached]
fn bit_indices(n: u16) -> Vec<usize> {
    (0..16).filter(|idx| (n >> idx) & 1 == 1).collect()
}

fn press_btn(cur_state: u16, btn: u16) -> u16 {
    cur_state ^ btn
}

fn parse(line: &str) -> Option<Mac> {
    let parts: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
    match parts.as_slice() {
        [lights, btns @ .., joltages] => {
            let lights = lights
                .chars()
                .skip(1)
                .enumerate()
                .filter_map(|(i, c)| if c == '#' { Some(i as u16) } else { None })
                .collect::<Vec<_>>();
            let mut btns = btns
                .iter()
                .map(|b| {
                    let ns = b[1..b.len() - 1]
                        .split(",")
                        .map(|n| n.parse::<u16>().unwrap())
                        .collect::<Vec<_>>();
                    ns_to_bitmask(&ns)
                })
                .collect::<Vec<_>>();
            // btns.sort_by_key(|b| -1 * *b as isize);
            let joltages = joltages[1..joltages.len() - 1]
                .split(",")
                .map(|n| n.parse::<u16>().unwrap())
                .collect::<Vec<_>>();
            Some(Mac {
                lights: ns_to_bitmask(&lights),
                btns,
                joltages,
            })
        }
        _ => None,
    }
}

fn find_least_btn_presses_pt1(mac: &Mac) -> Option<usize> {
    // (n_presses, btn_state).
    let mut q = VecDeque::from([(0usize, 0u16)]);
    let mut seen = HashSet::new();

    while !q.is_empty() {
        let (n, state) = q.pop_front().unwrap();
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state);
        if state == mac.lights {
            return Some(n);
        }
        for b in mac.btns.iter() {
            q.push_back((n + 1, press_btn(state, *b)));
        }
    }
    None
}

// Trying to go from 0 -> joltage requirement while BFS'ing.
fn find_least_btn_presses_pt2(mac: &Mac) -> Option<usize> {
    // (n_presses, joltages).
    let mut q = VecDeque::from([(0usize, vec![0u16; mac.joltages.len()])]);
    let mut seen = HashSet::new();

    while !q.is_empty() {
        let (n, joltages) = q.pop_front().unwrap();
        if seen.contains(&joltages) {
            continue;
        }
        if joltages
            .iter()
            .zip(mac.joltages.iter())
            .all(|(a, b)| a == b)
        {
            return Some(n);
        }

        // If we overpressed, skip.
        if joltages.iter().zip(mac.joltages.iter()).any(|(a, b)| a > b) {
            continue;
        }

        seen.insert(joltages.clone());

        for b in mac.btns.iter() {
            let mut joltages = joltages.clone();
            for pos in bit_indices(*b) {
                joltages[pos] += 1;
            }
            q.push_back((n + 1, joltages));
        }
    }
    None
}

// Trying to go from the joltage -> 0 while BFS'ing.
fn find_least_btn_presses_pt22(mac: &Mac) -> Option<usize> {
    // (n_presses, joltages).
    let mut q = VecDeque::from([(0usize, mac.joltages.clone())]);
    let mut seen = HashSet::new();

    while !q.is_empty() {
        let (n, joltages) = q.pop_front()?;
        if seen.contains(&joltages) {
            continue;
        }

        if joltages.iter().sum::<u16>() == 0 {
            return Some(n);
        }

        seen.insert(joltages.clone());

        // Sort buttons by sum?
        'outer: for b in mac.btns.iter() {
            let mut joltages = joltages.clone();

            let idcs = bit_indices(*b);

            // Greedy approach does not work.
            // let n_possible_presses = joltages
            //     .iter()
            //     .enumerate()
            //     .filter(|(i, j)| idcs.contains(i))
            //     .map(|(i, v)| *v)
            //     .min()
            //     .unwrap();

            // if n_possible_presses < 1 {
            //     continue 'outer;
            // }

            let n_possible_presses = 1;

            // println!("Possible to press {n_possible_presses} times");

            for pos in idcs {
                if joltages[pos] < n_possible_presses {
                    continue 'outer;
                }
                joltages[pos] -= n_possible_presses;
            }
            q.push_back((n + n_possible_presses as usize, joltages));
        }
    }
    None
}

fn solve_min_presses_z3(m: &Mac) -> Option<i64> {
    let solver = Solver::new();

    let n_presses = m
        .btns
        .iter()
        .enumerate()
        .map(|(i, b)| Int::fresh_const(&format!("b{i}")))
        .collect::<Vec<_>>();

    for n in &n_presses {
        solver.assert(n.ge(0));
    }

    for (idx, v) in m.joltages.iter().enumerate() {
        let v = Int::from_i64(*v as i64);
        // Relevant buttons.
        let relevant_btns: Vec<_> = m
            .btns
            .iter()
            .enumerate()
            .filter_map(|(i, b)| {
                if (b >> idx) & 1 == 1 {
                    Some(&n_presses[i])
                } else {
                    None
                }
            })
            .collect();

        if relevant_btns.is_empty() {
            continue;
        }

        let sum = Int::add(&relevant_btns);

        solver.assert(&sum.eq(&v));
    }

    // for sol in solver.solutions([alice, bob, charlie], false) {
    // for sol in solver.solutions(&n_presses, false).into_iter().take(100) {
    //     println!("{:?}", sol);
    // }

    let sols = solver.solutions(&n_presses, false).into_iter();

    sols.map(|s| {
        let values: Vec<i64> = s.iter().map(|v| v.as_i64().unwrap()).collect();
        values.iter().sum::<i64>()
    })
    .min()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(&filename).unwrap();

    let macs = contents
        .lines()
        .map(|l| parse(l).unwrap())
        .collect::<Vec<_>>();

    let presses = macs
        .iter()
        .map(|m| find_least_btn_presses_pt1(m).unwrap())
        .sum::<usize>();
    println!("{presses}");

    // Too slow.
    // let n = find_least_btn_presses_pt22(&macs[0]).unwrap();
    // println!("ok: {n}");

    let min_presses: i64 = macs
        .iter()
        .map(|m| {
            let min = solve_min_presses_z3(m).unwrap();
            println!("Found: {}", min);
            min
        })
        .sum();
    println!("{min_presses}");
}
