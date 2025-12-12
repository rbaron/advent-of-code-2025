use regex::Regex;

#[derive(Debug, Clone)]
struct Gift {
    size: u32,
}

impl Gift {
    fn parse(blk: &str) -> Option<Gift> {
        Some(Gift {
            size: blk.chars().filter(|c| *c == '#').count() as u32,
        })
    }

    fn area(&self) -> u32 {
        self.size
    }
}

#[derive(Debug, Clone)]
struct Area {
    w: u32,
    h: u32,
    pres: Vec<usize>,
}

impl Area {
    fn parse(line: &str) -> Option<Area> {
        let patt = Regex::new(
            r"(?<w>\d+)x(?<h>\d+): (?<n0>\d+) (?<n1>\d+) (?<n2>\d+) (?<n3>\d+) (?<n4>\d+) (?<n5>\d+)",
        )
        .unwrap();

        let caps = patt.captures(line)?;
        Some(Area {
            w: (&caps["w"]).parse().ok()?,
            h: (&caps["h"]).parse().ok()?,
            pres: (0..6)
                .map(|i| (&caps[format!("n{i}").as_str()]).parse().unwrap())
                .collect(),
        })
    }

    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn fits(&self, gifts: &[Gift]) -> bool {
        self.pres
            .iter()
            .zip(gifts)
            .map(|(n, g)| n * g.area() as usize)
            .sum::<usize>()
            <= self.area() as usize
    }
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(&filename).unwrap();

    let blks = contents.split("\n\n").collect::<Vec<_>>();

    let gifts = &blks[..blks.len() - 1]
        .iter()
        .map(|b| Gift::parse(*b).unwrap())
        .into_iter()
        .collect::<Vec<_>>();

    let areas: Vec<Area> = blks
        .last()
        .unwrap()
        .lines()
        .map(|l| Area::parse(l).unwrap())
        .collect::<Vec<_>>();

    let fit_areas = areas.iter().filter(|a| a.fits(gifts));

    println!(
        "Total areas: {}; fit areas: {}",
        areas.len(),
        fit_areas.count()
    );
}
