pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut grid = Grid::from_str(input);

    let mut nb_flashes = 0;
    for _ in 0..100 {
        nb_flashes += grid.simulate_one_turn();
    }
    nb_flashes
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut grid = Grid::from_str(input);

    let mut turn = 1;
    loop {
        if grid.simulate_one_turn() == (grid.width * grid.height) as u32 {
            return turn;
        }
        turn += 1;
    }
}

struct Grid {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let grid: Vec<Vec<u8>> = input
            .lines()
            .map(|line| line.bytes().map(|v| v - b'0').collect())
            .collect();
        let width = grid[0].len();
        let height = grid.len();

        Self {
            grid,
            width,
            height,
        }
    }

    fn simulate_one_turn(&mut self) -> u32 {
        // increase all values by 1
        for line in &mut self.grid {
            for v in line {
                *v += 1;
            }
        }

        let mut nb_flashes = 0;
        // loop until there are no more flashes
        loop {
            let mut had_flashes = false;
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.grid[y][x] > 9 {
                        had_flashes = true;
                        nb_flashes += 1;
                        self.grid[y][x] = 0;
                        if x > 0 {
                            if y > 0 {
                                self.bump(x - 1, y - 1)
                            }
                            self.bump(x - 1, y);
                            if y < self.height - 1 {
                                self.bump(x - 1, y + 1);
                            }
                        }
                        if y > 0 {
                            self.bump(x, y - 1)
                        }
                        if y < self.height - 1 {
                            self.bump(x, y + 1);
                        }
                        if x < self.width - 1 {
                            if y > 0 {
                                self.bump(x + 1, y - 1)
                            }
                            self.bump(x + 1, y);
                            if y < self.height - 1 {
                                self.bump(x + 1, y + 1);
                            }
                        }
                    }
                }
            }

            if !had_flashes {
                break;
            }
        }
        nb_flashes
    }

    fn bump(&mut self, x: usize, y: usize) {
        if self.grid[y][x] != 0 {
            self.grid[y][x] += 1;
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        for line in &self.grid {
            for c in line {
                f.write_char((*c + b'0') as char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
