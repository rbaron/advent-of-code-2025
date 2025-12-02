use std::collections::HashSet;

fn parse(contents: &str) -> Option<Vec<(u64, u64)>> {
    contents
        .split(",")
        .map(|r| {
            let (a, b) = r.split_once("-")?;
            Some((a.parse().ok()?, b.parse().ok()?))
        })
        .collect()
}

fn is_invalid_pt1(id: u64) -> bool {
    let n_digits = (id as f32).log10().floor() as u32 + 1;
    (id / 10u64.pow(n_digits / 2)) == (id % 10u64.pow(n_digits / 2))
}

fn is_repetition(mut id: u64, n_digits: u32, rep_size: u32) -> bool {
    if n_digits % rep_size != 0 {
        return false;
    }
    let div = 10u64.pow(rep_size);
    let mut v = HashSet::new();
    loop {
        if id == 0 {
            break;
        }
        v.insert(id % div);
        id /= div;
    }
    v.len() == 1
}

fn is_invalid_pt2(id: u64) -> bool {
    // Repetition must be at most n_digits / 2 in length (so there are at least 2).
    let n_digits = (id as f32).log10().floor() as u32 + 1;
    (1..=n_digits / 2).any(|rep_size| is_repetition(id, n_digits, rep_size))
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(&filename).unwrap();

    let ranges = parse(&contents).unwrap();

    let sum: u64 = ranges
        .iter()
        .flat_map(|&(lo, hi)| (lo..=hi).filter(|n| is_invalid_pt1(*n)))
        .sum();
    println!("{sum}");

    let sum: u64 = ranges
        .iter()
        .flat_map(|&(lo, hi)| (lo..=hi).filter(|n| is_invalid_pt2(*n)))
        .sum();
    println!("{sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_pt1() {
        assert_eq!(is_invalid_pt1(2002), false);
        assert_eq!(is_invalid_pt1(2020), true);
        assert_eq!(is_invalid_pt1(101), false);
        assert_eq!(is_invalid_pt1(1188511885), true);
        assert_eq!(is_invalid_pt1(1188511888), false);
    }

    #[test]
    fn test_is_repetition() {
        assert_eq!(is_repetition(22, 2, 1), true);
        assert_eq!(is_repetition(23, 2, 1), false);
        assert_eq!(is_repetition(8080, 4, 2), true);
        assert_eq!(is_repetition(8080, 4, 1), false);
        assert_eq!(is_repetition(8080, 4, 3), false);
        assert_eq!(is_repetition(9009009, 7, 3), false);
        assert_eq!(is_repetition(2121212121, 10, 2), true);
    }

    #[test]
    fn test_is_invalid_pt2() {
        assert_eq!(is_invalid_pt2(22), true);
        assert_eq!(is_invalid_pt2(1188511885), true);
        assert_eq!(is_invalid_pt2(1188511886), false);
        assert_eq!(is_invalid_pt2(9009009), false);
        assert_eq!(is_invalid_pt2(2121212121), true);
    }
}
