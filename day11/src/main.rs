use cached::proc_macro::cached;
use std::collections::{HashMap, HashSet};

type G<'a> = HashMap<&'a str, Vec<&'a str>>;

fn paths_to_out(start: &str, g: &G) -> usize {
    if start == "out" {
        return 1;
    }
    g[start].iter().map(|nxt| paths_to_out(nxt, g)).sum()
}

#[cached(
    key = "(String, bool, bool)",
    convert = r#"{ (start.to_string(), has_dac, has_fft) }"#
)]
fn paths_to_out_with_dac_fft(start: &str, g: &G, has_dac: bool, has_fft: bool) -> usize {
    if start == "out" {
        return if has_dac && has_fft { 1 } else { 0 };
    } else {
        0;
    }
    g[start]
        .iter()
        .map(|nxt| {
            paths_to_out_with_dac_fft(nxt, g, has_dac || start == "dac", has_fft || start == "fft")
        })
        .sum()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(&filename).unwrap();

    let g: G = G::from_iter(contents.lines().map(|l| {
        let parts = l.split(" ").collect::<Vec<_>>();
        (&parts[0][..(parts[0].len() - 1)], parts[1..].to_vec())
    }));

    let pt1 = paths_to_out("you", &g);
    println!("{pt1}");

    let pt2 = paths_to_out_with_dac_fft("svr", &g, false, false);
    println!("{pt2}");
}
