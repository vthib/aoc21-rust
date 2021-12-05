pub fn part1(input: &str) -> impl std::fmt::Display {
    let lines = parse_input(input);

    let mut grid = Grid::new(&lines);
    for line in lines {
        grid.add_line(&line, false);
    }

    grid.get_score()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let lines = parse_input(input);

    let mut grid = Grid::new(&lines);
    for line in lines {
        grid.add_line(&line, true);
    }

    grid.get_score()
}

struct Grid(Vec<Vec<u32>>);

impl Grid {
    fn new(lines: &[Line]) -> Self {
        // Create grid fitting all lines, initialize it to 0 everywhere.
        let max_x = lines.iter().map(|line| line.to.x).max().unwrap();
        let max_y = lines
            .iter()
            .map(|line| std::cmp::max(line.from.y, line.to.y))
            .max()
            .unwrap();
        let mut grid = Vec::new();
        for _ in 0..(max_y + 1) {
            grid.push(vec![0; (max_x + 1) as usize]);
        }
        Self(grid)
    }

    fn get_score(&self) -> u32 {
        self.0
            .iter()
            .map(|line| {
                line.iter()
                    .fold(0, |acc, v| if *v >= 2 { acc + 1 } else { acc })
            })
            .sum()
    }

    /// Add 1 in every cell covered by the line.
    fn add_line(&mut self, line: &Line, include_diagonals: bool) {
        if line.from.x == line.to.x {
            // vertical line
            let x = line.from.x as usize;

            if line.from.y < line.to.y {
                for y in line.from.y..=line.to.y {
                    self.0[y as usize][x] += 1;
                }
            } else {
                for y in line.to.y..=line.from.y {
                    self.0[y as usize][x] += 1;
                }
            }
        } else if line.to.y == line.from.y {
            // horizontal line
            let y = line.from.y as usize;

            for x in line.from.x..=line.to.x {
                self.0[y][x as usize] += 1;
            }
        } else {
            if !include_diagonals {
                return;
            }

            // Diagonals can be in 4 directions, but lines
            // have been normalized with increasing x.
            // So there are only two cases:
            // - increasing x, decreasing y
            // - increasing x, increasing y
            let from_x = line.from.x as usize;
            let from_y = line.from.y as usize;
            let len = line.to.x - line.from.x + 1;
            if line.from.y < line.to.y {
                for i in 0..len {
                    self.0[from_y + i as usize][from_x + i as usize] += 1;
                }
            } else {
                for i in 0..len {
                    self.0[from_y - i as usize][from_x + i as usize] += 1;
                }
            }
        }
    }
}
fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(Line::from_str).collect()
}

#[derive(Debug)]
struct Line {
    from: Coords,
    to: Coords,
}

impl Line {
    fn from_str(input: &str) -> Self {
        let mut coords = input.split(" -> ");
        let from = coords.next().unwrap();
        let to = coords.next().unwrap();
        assert!(coords.next().is_none());

        let mut from = Coords::from_str(from);
        let mut to = Coords::from_str(to);

        // Normalize directions: always have increasing X,
        // then if possible increasing Y
        if from.x > to.x || (from.y > to.y && from.x == to.x) {
            std::mem::swap(&mut from, &mut to);
        }

        Line { from, to }
    }
}

#[derive(Debug)]
struct Coords {
    x: u32,
    y: u32,
}

impl Coords {
    fn from_str(input: &str) -> Self {
        let mut values = input.split(',');

        Self {
            x: values.next().unwrap().parse().unwrap(),
            y: values.next().unwrap().parse().unwrap(),
        }
    }
}
