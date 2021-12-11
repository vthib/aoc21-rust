use std::collections::{HashSet, VecDeque};

pub fn part1(input: &str) -> impl std::fmt::Display {
    let heightmap = parse_input(input);

    let height = heightmap.len();
    assert!(!heightmap.is_empty());
    let width = heightmap[0].len();
    assert!(heightmap.iter().all(|line| line.len() == width));

    let mut score: u32 = 0;
    for (y, line) in heightmap.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            let local_low = (x == 0 || heightmap[y][x - 1] > *value)
                && (x == width - 1 || heightmap[y][x + 1] > *value)
                && (y == 0 || heightmap[y - 1][x] > *value)
                && (y == height - 1 || heightmap[y + 1][x] > *value);
            if local_low {
                score += (*value as u32) + 1;
            }
        }
    }
    score
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let heightmap = parse_input(input);

    let height = heightmap.len();
    assert!(!heightmap.is_empty());
    let width = heightmap[0].len();
    assert!(heightmap.iter().all(|line| line.len() == width));

    let mut basins = Vec::new();
    for (y, line) in heightmap.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            let local_low = (x == 0 || heightmap[y][x - 1] > *value)
                && (x == width - 1 || heightmap[y][x + 1] > *value)
                && (y == 0 || heightmap[y - 1][x] > *value)
                && (y == height - 1 || heightmap[y + 1][x] > *value);
            if local_low {
                basins.push(get_basin_size(&heightmap, x, y, width, height));
            }
        }
    }

    basins.sort_unstable();
    basins.iter().rev().take(3).product::<u32>()
}

fn get_basin_size(grid: &[Vec<u8>], x: usize, y: usize, width: usize, height: usize) -> u32 {
    // breadth first search to compute basin
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();

    to_visit.push_back((x, y));
    while let Some((x, y)) = to_visit.pop_front() {
        if !visited.insert((x, y)) {
            continue;
        }

        let v = grid[y][x];
        if x > 0 && grid[y][x - 1] > v && grid[y][x - 1] != 9 {
            to_visit.push_back((x - 1, y));
        }
        if x < width - 1 && grid[y][x + 1] > v && grid[y][x + 1] != 9 {
            to_visit.push_back((x + 1, y));
        }
        if y > 0 && grid[y - 1][x] > v && grid[y - 1][x] != 9 {
            to_visit.push_back((x, y - 1));
        }
        if y < height - 1 && grid[y + 1][x] > v && grid[y + 1][x] != 9 {
            to_visit.push_back((x, y + 1));
        }
    }
    visited.len() as u32
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|v| v - b'0').collect())
        .collect()
}
