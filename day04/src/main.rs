use itertools::iproduct;
use num::complex::Complex;
use std::collections::{HashMap, HashSet};

type Coord = Complex<i32>;
type Grid = HashMap<Coord, char>;

fn parse(contents: &str) -> Grid {
    HashMap::from_iter(contents.lines().enumerate().flat_map(|(y, l)| {
        l.chars()
            .enumerate()
            .map(move |(x, c)| (Complex::new(x as i32, y as i32), c))
    }))
}

fn neighbors(coord: &Coord) -> impl Iterator<Item = Coord> {
    iproduct!([-1, 0, 1], [-1, 0, 1]).filter_map(move |c| match c {
        (0, 0) => None,
        (x, y) => Some(coord + Complex::new(x, y)),
    })
}

fn count_stacks(coord: &Coord, grid: &Grid) -> usize {
    neighbors(coord)
        .map(|n| match grid.get(&n) {
            Some('@') => 1,
            _ => 0,
        })
        .sum()
}

fn find_removable(grid: &Grid) -> HashSet<Coord> {
    HashSet::from_iter(
        grid.iter()
            .filter(move |(k, v)| **v == '@' && count_stacks(k, &grid) < 4)
            .map(|(k, _)| *k),
    )
}

fn step_remove(grid: &Grid, removable: &HashSet<Coord>) -> Grid {
    Grid::from_iter(
        grid.clone()
            .into_iter()
            .filter(|(k, _)| !removable.contains(k)),
    )
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(&filename).unwrap();
    let grid = parse(&contents);

    let avail = find_removable(&grid).len();
    println!("{:?}", avail);

    let mut count: usize = 0;
    let mut grid = grid;
    loop {
        let removable = find_removable(&grid);
        count += removable.len();

        if removable.len() == 0 {
            break;
        }

        grid = step_remove(&grid, &removable);
    }

    println!("{}", count);
}
