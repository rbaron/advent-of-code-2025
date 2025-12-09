use itertools::Itertools;
use plotters::{prelude::*, style::full_palette::PINK_100};

#[derive(Debug, Clone, Copy)]
struct Pt {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Seg {
    lo: Pt,
    hi: Pt,
}

impl Seg {
    fn is_horiz(&self) -> bool {
        self.lo.y == self.hi.y
    }
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    // Top right.
    ctop: Pt,
    // Bottom left.
    cbot: Pt,
}

impl Rect {
    fn new(c1: &Pt, c2: &Pt) -> Rect {
        let minx = c1.x.min(c2.x);
        let maxx = c1.x.max(c2.x);
        let miny = c1.y.min(c2.y);
        let maxy = c1.y.max(c2.y);

        Rect {
            ctop: Pt { x: maxx, y: maxy },
            cbot: Pt { x: minx, y: miny },
        }
    }
    fn lo_horiz(&self) -> Seg {
        Seg {
            lo: self.cbot,
            hi: Pt {
                x: self.ctop.x,
                y: self.cbot.y,
            },
        }
    }
    fn hi_horiz(&self) -> Seg {
        Seg {
            lo: Pt {
                x: self.cbot.x,
                y: self.ctop.y,
            },
            hi: self.ctop,
        }
    }
    fn hi_v(&self) -> Seg {
        Seg {
            lo: Pt {
                x: self.ctop.x,
                y: self.cbot.y,
            },
            hi: self.ctop,
        }
    }
    fn area(&self) -> i64 {
        (self.ctop.x - self.cbot.x + 1).abs() as i64 * (self.ctop.y - self.cbot.y + 1).abs() as i64
    }
}

fn crosses(hor: &Seg, ver: &Seg) -> bool {
    hor.lo.x < ver.hi.x && hor.hi.x > ver.hi.x && ver.lo.y < hor.lo.y && ver.hi.y > hor.lo.y
}

fn plot_to_png(pts: &[Pt], rects: &[Rect]) -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;

    let root = BitMapBackend::new("plot.png", (1800, 1800)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_max = pts.iter().map(|p| p.x).max().unwrap_or(1);
    let y_max = pts.iter().map(|p| p.y).max().unwrap_or(1);

    let x_max = x_max as f32;
    let y_max = y_max as f32;

    let mut chart = ChartBuilder::on(&root)
        .caption("Plot with Rectangles", ("sans-serif", 30))
        .margin(10)
        .set_left_and_bottom_label_area_size(40)
        .build_cartesian_2d(0f32..x_max, 0f32..y_max)?;

    chart.configure_mesh().draw()?;

    if pts.len() >= 2 {
        chart.draw_series(LineSeries::new(
            pts.iter().map(|p| (p.x as f32, p.y as f32)),
            &BLUE,
        ))?;
    }

    chart.draw_series(pts.iter().enumerate().map(|(i, p)| {
        let color = if i == 0 {
            RGBColor(255, 105, 180) // pink
        } else if i == pts.len() - 1 {
            GREEN
        } else {
            RED
        };
        Circle::new((p.x as f32, p.y as f32), 6, color.filled())
    }))?;

    chart.draw_series(rects.iter().map(|r| {
        Rectangle::new(
            [
                (r.ctop.x as f32, r.ctop.y as f32),
                (r.cbot.x as f32, r.cbot.y as f32),
            ],
            RGBColor(255, 255, 0).mix(0.5).filled(),
        )
    }))?;

    for r in rects {
        chart.draw_series(
            rects
                .iter()
                .map(|r| Circle::new((r.ctop.x as f32, r.ctop.y as f32), 6, GREEN.filled())),
        )?;
    }

    for r in rects {
        chart.draw_series(
            rects
                .iter()
                .map(|r| Circle::new((r.cbot.x as f32, r.cbot.y as f32), 6, PINK_100.filled())),
        )?;
    }

    root.present()?;
    Ok(())
}

fn find_largest_area(pts: &[Pt]) -> Option<i64> {
    let pairs = pts.iter().combinations(2).map(|v| match v.as_slice() {
        [c1, c2] => Rect::new(*c1, *c2).area(),
        _ => panic!(),
    });
    pairs.max()
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let contents = std::fs::read_to_string(&filename).unwrap();

    let pts = contents
        .lines()
        .map(|l| {
            let ns = l.split_once(",").unwrap();
            Pt {
                x: ns.0.parse::<i32>().unwrap(),
                y: ns.1.parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    // let segs = pts.iter().zip(pts.iter().skip(1));

    // Pt1.
    println!("{}", find_largest_area(&pts).unwrap());

    // plot_to_png(&pts).unwrap();

    // The set of points is mostly concave (with some small convex curves at the border),
    // except for two points opposite to the origin, which "cut" through the inner area.
    // Probably the largest inside area will be either above or below this cut.
    // On my input:
    /*
    ...
    2503,52132
    2179,52132
    2179,50928
    1712,50928
    1712,50273
    94737,50273    -> First jump accurs here.
    94737,48494
    2140,48494     -> Then it jumps back here.
    2140,47295
    2419,47295
    2419,46101
    ...
    */

    // One idea is to split the points into two sets:
    // 1. Points with y >= 50273
    // 2. Points with y <= 48494

    let p1 = Pt { x: 94737, y: 50273 };

    let pts1 = pts.iter().filter(|p| p.y >= p1.y).collect::<Vec<_>>();
    let segs = pts1
        .iter()
        .zip(pts1.iter().skip(1))
        .map(|(p1, p2)| Seg {
            lo: Pt {
                x: p1.x.min(p2.x),
                y: p1.y.min(p2.y),
            },
            hi: Pt {
                x: p1.x.max(p2.x),
                y: p1.y.max(p2.y),
            },
        })
        .collect::<Vec<_>>();

    let v_segs = segs.iter().filter(|s| !s.is_horiz()).collect::<Vec<_>>();
    let h_segs = segs.iter().filter(|s| s.is_horiz()).collect::<Vec<_>>();

    let rects = pts1.iter().map(|p| Rect::new(p, &p1));

    let remaining_rects = rects
        .filter(|r| {
            let rh = r.lo_horiz();
            let rv = r.hi_v();
            v_segs.iter().all(|v| !crosses(&rh, &v)) && h_segs.iter().all(|h| !crosses(&h, &rv))
        })
        .collect::<Vec<_>>();

    let r_p1 = remaining_rects.iter().max_by_key(|r| r.area()).unwrap();

    println!("rect {:?}", r_p1.area());

    let p2 = Pt { x: 94737, y: 48494 };

    let pts2 = pts.iter().filter(|p| p.y <= p2.y).collect::<Vec<_>>();
    let segs = pts2
        .iter()
        .zip(pts2.iter().skip(1))
        .map(|(p1, p2)| Seg {
            lo: Pt {
                x: p1.x.min(p2.x),
                y: p1.y.min(p2.y),
            },
            hi: Pt {
                x: p1.x.max(p2.x),
                y: p1.y.max(p2.y),
            },
        })
        .collect::<Vec<_>>();

    let v_segs = segs.iter().filter(|s| !s.is_horiz()).collect::<Vec<_>>();
    let h_segs = segs.iter().filter(|s| s.is_horiz()).collect::<Vec<_>>();

    let rects = pts2.iter().map(|p| Rect::new(p, &p1));

    let remaining_rects = rects
        .filter(|r| {
            let rh = r.lo_horiz();
            let rv = r.hi_v();
            v_segs.iter().all(|v| !crosses(&rh, &v)) && h_segs.iter().all(|h| !crosses(&h, &rv))
        })
        .collect::<Vec<_>>();

    let r_p2 = remaining_rects.iter().max_by_key(|r| r.area()).unwrap();

    println!("rect {:?}", r_p2.area());

    println!("Pt2: {:?}", r_p1.area().max(r_p2.area()));

    plot_to_png(&pts, &[*r_p1, *r_p2]).unwrap();

    // 3041911911 too high.
    // 2871878928 too high.
    // 1207618188 incorrect.
    // 1454690654 incorrect.
    // 1291825827 p2 incorrect.
    // 1470175787 too low.
    // 2825252256 incorrect.
    // 1539204402 incorrect.

    // 1539238860 bingo hell yeah first try
}
