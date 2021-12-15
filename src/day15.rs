use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let (risks, width, height) = parse_input(input);

    let dists = compute_distances(&risks, width, height);

    dists[(height - 1) * width + width - 1]
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let (risks, width, height) = parse_input(input);

    let risks5: Vec<Vec<u32>> = (0..height * 5)
        .map(|j| {
            let j_add = (j / height) as u32;
            let j = j % height;

            (0..width * 5)
                .map(|i| {
                    let new_value = if i < width {
                        risks[j][i] + j_add
                    } else {
                        risks[j][i % width] + ((i / width) as u32) + j_add
                    };
                    if new_value >= 19 {
                        new_value - 18
                    } else if new_value >= 10 {
                        new_value - 9
                    } else {
                        new_value
                    }
                })
                .collect::<Vec<u32>>()
        })
        .collect();
    let dists = compute_distances(&risks5, width * 5, height * 5);

    dists[(height * 5 - 1) * width * 5 + width * 5 - 1]
}

fn compute_distances(risks: &[Vec<u32>], width: usize, height: usize) -> Vec<u32> {
    // Do Dijkstra's algorithm to find shortest path
    let mut heap = BinaryHeap::new();

    // current shortest distance from 0,0 to y*width + x
    let mut dists = vec![u32::MAX; width * height];

    dists[0] = 0;
    heap.push(Reverse(CellRisk {
        x: 0,
        y: 0,
        risk: 0,
    }));

    // pop, compute distance to neighbor, repeat
    while let Some(cell) = heap.pop() {
        let cell = cell.0;

        if cell.x == width - 1 && cell.y == height - 1 {
            break;
        }
        let pos = cell.y * width + cell.x;
        let CellRisk {
            x,
            y,
            risk: prev_risk,
        } = cell;

        // We already have a better path to this position, so ignore
        if prev_risk > dists[pos] {
            continue;
        }

        if x > 0 {
            let pos = y * width + x - 1;
            let risk = prev_risk + risks[y][x - 1];
            if risk < dists[pos] {
                dists[pos] = risk;
                heap.push(Reverse(CellRisk { x: x - 1, y, risk }));
            }
        }
        if x < width - 1 {
            let pos = y * width + x + 1;
            let risk = prev_risk + risks[y][x + 1];
            if risk < dists[pos] {
                dists[pos] = risk;
                heap.push(Reverse(CellRisk { x: x + 1, y, risk }));
            }
        }
        if y > 0 {
            let pos = (y - 1) * width + x;
            let risk = prev_risk + risks[y - 1][x];
            if risk < dists[pos] {
                dists[pos] = risk;
                heap.push(Reverse(CellRisk { x, y: y - 1, risk }));
            }
        }
        if y < height - 1 {
            let pos = (y + 1) * width + x;
            let risk = prev_risk + risks[y + 1][x];
            if risk < dists[pos] {
                dists[pos] = risk;
                heap.push(Reverse(CellRisk { x, y: y + 1, risk }));
            }
        }
    }
    dists
}

fn parse_input(input: &str) -> (Vec<Vec<u32>>, usize, usize) {
    let res: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.bytes().map(|v| (v - b'0') as u32).collect())
        .collect();
    let height = res.len();
    let width = res[0].len();
    assert!(res.iter().all(|v| v.len() == width));
    (res, width, height)
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct CellRisk {
    // Put risk first to have it used for ordering in priority
    risk: u32,
    x: usize,
    y: usize,
}
