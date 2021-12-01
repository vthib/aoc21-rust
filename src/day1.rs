pub fn part1(input: &str) -> impl std::fmt::Display {
    // Parse depth from input: each line should contain a number and only a number.
    let depths: Vec<u32> = input.lines().map(|v| v.parse().unwrap()).collect();

    compute_nb_increases(&depths)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    // Parse depth from input: each line should contain a number and only a number.
    let depths: Vec<u32> = input.lines().map(|v| v.parse().unwrap()).collect();

    // Compute the sums of each subsequent window of 3 values
    let sums_of_3_depths: Vec<_> = depths.windows(3).map(|v| v.iter().sum()).collect();

    compute_nb_increases(&sums_of_3_depths)
}

fn compute_nb_increases(v: &[u32]) -> u32 {
    // count the number of increases in the slice
    v.windows(2).fold(
        0,
        |acc, values| if values[0] < values[1] { acc + 1 } else { acc },
    )
}
