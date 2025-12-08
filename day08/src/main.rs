use std::mem;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Pt {
    idx: usize,
    x: i64,
    y: i64,
    z: i64,
}

impl Pt {
    fn parse(idx: usize, l: &str) -> Option<Pt> {
        let ns: Vec<_> = l.split(",").map(|n| n.parse::<i64>()).collect();
        if ns.len() != 3 {
            return None;
        }
        Some(Pt {
            idx,
            x: *ns[0].as_ref().ok()?,
            y: *ns[1].as_ref().ok()?,
            z: *ns[2].as_ref().ok()?,
        })
    }

    fn dist(&self, Pt { x, y, z, .. }: &Self) -> i64 {
        ((self.x - x).pow(2) + (self.y - y).pow(2) + (self.z - z).pow(2)).isqrt()
    }
}

fn parse_all(contents: &str) -> Option<Vec<Pt>> {
    contents
        .lines()
        .enumerate()
        .map(|(idx, l)| Pt::parse(idx, l))
        .collect()
}

#[derive(Debug, Clone)]
struct UnionFind {
    parent_by_idx: Vec<usize>,
    size_by_idx: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind {
            parent_by_idx: Vec::from_iter(0..size),
            size_by_idx: vec![1; size],
        }
    }

    fn find(&self, idx: usize) -> usize {
        let mut x = idx;
        while self.parent_by_idx[x] != x {
            x = self.parent_by_idx[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut pa = self.find(a);
        let mut pb = self.find(b);
        // Already connected.
        if pa == pb {
            return;
        }

        // Union by size.
        if self.size_by_idx[pa] < self.size_by_idx[pb] {
            mem::swap(&mut pa, &mut pb);
        }
        self.parent_by_idx[pb] = pa;
        self.size_by_idx[pa] += self.size_by_idx[pb];
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(&filename).unwrap();

    let pts = parse_all(&contents).unwrap();

    let mut combs = pts
        .iter()
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .collect::<Vec<_>>();

    combs.sort_by_key(|(a, b)| a.dist(b));

    let mut uf = UnionFind::new(pts.len());

    for (a, b) in combs.iter().take(1000) {
        uf.union(a.idx, b.idx);
    }

    let mut sizes = uf.size_by_idx;
    sizes.sort();
    let r: usize = sizes.iter().rev().take(3).product();
    println!("{r}");

    let mut uf = UnionFind::new(pts.len());
    for (a, b) in combs {
        uf.union(a.idx, b.idx);
        if uf.size_by_idx.iter().max().unwrap() == &pts.len() {
            println!("{}", a.x * b.x);
            break;
        }
    }
}
