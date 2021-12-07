pub fn part1(input: &str) -> impl std::fmt::Display {
    let (positions, min, max) = parse_input(input);

    (min..=max)
        .map(|index| positions.iter().fold(0, |acc, v| acc + (*v - index).abs()))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let (positions, min, max) = parse_input(input);

    (min..=max)
        .map(|index| {
            positions.iter().fold(0, |acc, v| {
                let distance = (*v - index).abs();
                acc + distance * (distance + 1) / 2
            })
        })
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> (Vec<i32>, i32, i32) {
    let positions: Vec<_> = input
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (positions, min, max)
}
