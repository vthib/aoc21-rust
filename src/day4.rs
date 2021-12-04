pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut game = parse_input(input);

    for picked_number in &game.picked_numbers {
        for grid in game.grids.iter_mut() {
            if let Some(score) = grid.pick_number(*picked_number) {
                return score;
            }
        }
    }
    unreachable!()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut game = parse_input(input);

    let mut nb_grids_left = game.grids.len();
    for picked_number in &game.picked_numbers {
        for grid in game.grids.iter_mut() {
            if grid.complete {
                continue;
            }
            if let Some(score) = grid.pick_number(*picked_number) {
                if nb_grids_left == 1 {
                    return score;
                }
                nb_grids_left -= 1;
            }
        }
    }
    unreachable!()
}
#[derive(Debug)]
struct BingoGame {
    picked_numbers: Vec<i8>,
    grids: Vec<Grid>,
}

#[derive(Debug)]
struct Grid {
    lines: Vec<Vec<i8>>,
    complete: bool,
}

impl Grid {
    fn pick_number(&mut self, picked_number: i8) -> Option<u32> {
        let mut has_picked = false;

        for line in self.lines.iter_mut() {
            for v in line.iter_mut() {
                if *v == picked_number {
                    *v = -1;
                    // Do not check & return here: the picked number may need
                    // to be removed in other lines.
                    has_picked = true;
                }
            }
        }

        if has_picked && self.check_completion() {
            let grid_score: u32 = self
                .lines
                .iter()
                .map(|line| {
                    line.iter()
                        .filter_map(|v| if *v != -1 { Some(*v as u32) } else { None })
                        .sum::<u32>()
                })
                .sum();
            Some(picked_number as u32 * grid_score)
        } else {
            None
        }
    }

    fn check_completion(&mut self) -> bool {
        // Check horizontal lines
        if self.lines.iter().any(|line| line.iter().all(|v| *v == -1)) {
            self.complete = true;
            return true;
        }

        // Check vertical lines
        let len = self.lines.len();
        for i in 0..len {
            if self.lines.iter().map(|line| line[i]).all(|v| v == -1) {
                self.complete = true;
                return true;
            }
        }

        false
    }
}

fn parse_input(input: &str) -> BingoGame {
    let mut lines = input.split("\n\n");

    let picked_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let grids = lines
        .map(|v| Grid {
            lines: v
                .lines()
                .map(|v| {
                    v.split_ascii_whitespace()
                        .map(|v| v.parse().unwrap())
                        .collect()
                })
                .collect(),
            complete: false,
        })
        .collect();

    BingoGame {
        picked_numbers,
        grids,
    }
}
