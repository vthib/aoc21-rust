pub fn part1(input: &str) -> impl std::fmt::Display {
    let (mut points, folds) = parse_input(input);

    assert!(!folds.is_empty());

    folds[0].apply_on_points(&mut points);
    points.sort_unstable();
    points.dedup();
    points.len()
}

#[allow(unused)]
pub fn part2(input: &str) -> impl std::fmt::Display {
    let (mut points, folds) = parse_input(input);

    for fold in folds {
        fold.apply_on_points(&mut points);
    }

    points.sort_unstable();
    points.dedup();
    display_points(&points);
    0
}

fn display_points(points: &[Point]) {
    let width = points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = points.iter().map(|p| p.y).max().unwrap() + 1;

    let mut points = points.iter();
    let mut point = points.next();
    for y in 0..height {
        for x in 0..width {
            let mut c = '.';

            if let Some(p) = point {
                if p.x == x && p.y == y {
                    point = points.next();
                    c = '#';
                }
            }
            print!("{}", c);
        }
        println!();
    }
}

fn parse_input(input: &str) -> (Vec<Point>, Vec<Fold>) {
    let mut elems = input.split("\n\n");
    let points = elems.next().unwrap();
    let folds = elems.next().unwrap();
    assert!(elems.next().is_none());

    (
        points
            .lines()
            .map(|line| {
                let mut elems = line.split(',');

                let x = elems.next().unwrap().parse().unwrap();
                let y = elems.next().unwrap().parse().unwrap();
                Point { x, y }
            })
            .collect(),
        folds
            .lines()
            .map(|line| {
                let line = line.strip_prefix("fold along ").unwrap();
                let mut elems = line.split('=');

                let direction = match elems.next().unwrap() {
                    "x" => FoldDirection::Vertical,
                    "y" => FoldDirection::Horizontal,
                    _ => unreachable!(),
                };
                let position = elems.next().unwrap().parse().unwrap();
                Fold {
                    direction,
                    position,
                }
            })
            .collect(),
    )
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    y: u32,
    x: u32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}

struct Fold {
    direction: FoldDirection,
    position: u32,
}

impl Fold {
    fn apply_on_points(&self, points: &mut [Point]) {
        match self.direction {
            FoldDirection::Horizontal => {
                for p in points.iter_mut() {
                    if p.y > self.position {
                        p.y = self.position - (p.y - self.position);
                    }
                }
            }
            FoldDirection::Vertical => {
                for p in points.iter_mut() {
                    if p.x > self.position {
                        p.x = self.position - (p.x - self.position);
                    }
                }
            }
        }
    }
}
enum FoldDirection {
    Horizontal,
    Vertical,
}
