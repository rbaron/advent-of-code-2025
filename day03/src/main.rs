use std::collections::{HashMap, VecDeque};

// Returns (max_pair, max_element).
fn find_max_joltage_pair(batts: &[i32]) -> (i32, i32) {
    if batts.len() == 1 {
        return (0, batts[0]);
    } else if batts.len() == 2 {
        return (batts[0] * 10 + batts[1], *batts.iter().max().unwrap());
    }

    // Otherwise we need to combine the solutions.
    let (left, right) = batts.split_at(batts.len() / 2);

    let (left_max_pair, left_max_el) = find_max_joltage_pair(left);
    let (right_max_pair, right_max_el) = find_max_joltage_pair(right);

    let pair_across = left_max_el * 10 + right_max_el;

    (
        *[left_max_pair, right_max_pair, pair_across]
            .iter()
            .max()
            .unwrap(),
        *[left_max_el, right_max_el].iter().max().unwrap(),
    )
}

fn calc_joltage(batts: &[i32]) -> u64 {
    batts
        .iter()
        .enumerate()
        .map(|(i, b)| (*b as u64) * 10u64.pow(batts.len() as u32 - 1 - i as u32))
        .sum()
}

fn find_max_joltage_with_budget(batts: &[i32], budget: i32) -> u64 {
    // (joltage, budget, batts slice).
    let mut q = VecDeque::from([(0, budget, batts)]);

    let mut max_by_budget: HashMap<i32, u64> = HashMap::new();

    while !q.is_empty() {
        let (joltage, budget, batts) = q.pop_front().unwrap();

        let m = max_by_budget.entry(budget).or_insert(0);
        if joltage < *m {
            continue;
        }

        *m = joltage;

        if budget == 0 {
            continue;
        }

        // Only one possible combination.
        if batts.len() == budget as usize {
            q.push_back((joltage + calc_joltage(batts), 0, &[]));
            continue;
        }

        let head = batts[0];
        // Solutions with head.
        q.push_back((
            joltage + head as u64 * 10u64.pow(budget as u32 - 1),
            budget - 1,
            &batts[1..],
        ));
        // Solutions without head.
        q.push_back((joltage, budget, &batts[1..]));
    }

    // current_max
    *max_by_budget.get(&0).unwrap()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(&filename).unwrap();

    let banks: Vec<Vec<i32>> = contents
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let joltage_pair: i32 = banks.iter().map(|b| find_max_joltage_pair(b).0).sum();
    println!("{joltage_pair}");

    let joltage_dozen: u64 = banks
        .iter()
        .map(|b| find_max_joltage_with_budget(b, 12))
        .sum();
    println!("{joltage_dozen}");
}
