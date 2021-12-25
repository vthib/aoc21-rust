pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut grid = parse_input(input);

    let mut nb_turns = 1;
    while grid.next_step() {
        nb_turns += 1;
    }
    nb_turns
}

#[derive(PartialEq, Copy, Clone)]
enum Cell {
    Empty,
    East,
    South,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(cells: Vec<Vec<Cell>>) -> Self {
        let width = cells[0].len();
        let height = cells.len();
        Self {
            cells,
            width,
            height,
        }
    }

    // Apply next step in grid. Returns true if the grid has been modified
    fn next_step(&mut self) -> bool {
        let mut modified = false;

        // Do east-moving cucumbers first.
        let mut to_move = Vec::with_capacity(self.width);
        for line in self.cells.iter_mut() {
            for i in 0..self.width {
                to_move.push(line[i] == Cell::East && line[(i + 1) % self.width] == Cell::Empty);
            }
            for i in 0..self.width {
                if to_move[i] {
                    line[i] = Cell::Empty;
                    line[(i + 1) % self.width] = Cell::East;
                    modified = true;
                }
            }
            to_move.clear();
        }

        // Then south-moving cucumbers.
        let mut to_move = Vec::with_capacity(self.height);
        for i in 0..self.width {
            for j in 0..self.height {
                to_move.push(
                    self.cells[j][i] == Cell::South
                        && self.cells[(j + 1) % self.height][i] == Cell::Empty,
                );
            }
            for j in 0..self.height {
                if to_move[j] {
                    self.cells[j][i] = Cell::Empty;
                    self.cells[(j + 1) % self.height][i] = Cell::South;
                    modified = true;
                }
            }
            to_move.clear();
        }
        modified
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in &self.cells {
            for c in line {
                write!(
                    f,
                    "{}",
                    match c {
                        Cell::Empty => '.',
                        Cell::East => '>',
                        Cell::South => 'v',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Grid {
    let byte_to_cell = |c| match c {
        b'.' => Cell::Empty,
        b'>' => Cell::East,
        b'v' => Cell::South,
        _ => unreachable!(),
    };

    Grid::new(
        input
            .lines()
            .map(|line| line.bytes().map(byte_to_cell).collect())
            .collect(),
    )
}
