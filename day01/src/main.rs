const N: i32 = 100;

fn count_crossings(pos: i32, dir: &str, amount: i32) -> (i32, i32) {
    let calc = |d_from_zero: i32, amount: i32, new_pos: i32| {
        if amount >= d_from_zero && d_from_zero > 0 {
            (new_pos, 1 + (amount - d_from_zero) / N)
        } else if d_from_zero == 0 {
            (new_pos, 0 + (amount - d_from_zero) / N)
        } else {
            (new_pos, 0)
        }
    };

    match dir {
        "R" => calc(N - pos, amount, (pos + amount).rem_euclid(N)),
        "L" => calc(pos, amount.abs(), (pos - amount).rem_euclid(N)),
        _ => panic!(),
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(&filename).unwrap();

    let ops = contents.lines().map(|l| {
        let (dir, amount) = l.split_at(1);
        (dir, amount.parse::<i32>().unwrap())
    });

    // Pt1.
    let (_, zeros) = ops.clone().fold((50, 0), |(pos, zeros), (dir, amount)| {
        let new_pos = match dir {
            "R" => (pos + amount).rem_euclid(N),
            "L" => (pos - amount).rem_euclid(N),
            _ => panic!(),
        };
        (new_pos, zeros + if new_pos == 0 { 1 } else { 0 })
    });
    println!("{zeros}");

    // Pt2.
    let (_, zeros) = ops.fold((50, 0), |(pos, zeros), (dir, amount)| {
        let (pos, cross) = count_crossings(pos, dir, amount);
        (pos, cross + zeros)
    });
    println!("{zeros}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crossings() {
        assert_eq!(count_crossings(50, "L", 68), (82, 1));
        assert_eq!(count_crossings(82, "L", 30), (52, 0));
        assert_eq!(count_crossings(52, "R", 48), (0, 1));
        assert_eq!(count_crossings(0, "L", 5), (95, 0));
        assert_eq!(count_crossings(95, "R", 60), (55, 1));
        assert_eq!(count_crossings(55, "L", 55), (0, 1));
    }
}
