pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut counts = parse_input(input);

    simulate_for(&mut counts, 80);
    counts.iter().sum::<u64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut counts = parse_input(input);

    simulate_for(&mut counts, 256);
    counts.iter().sum::<u64>()
}

fn parse_input(input: &str) -> [u64; 9] {
    // create 9 buckets, from 0 to 8, and fill from the input
    let mut counts = [0; 9];
    for v in input.split(',') {
        let n: usize = v.trim().parse().unwrap();

        assert!(n < 9);
        counts[n] += 1;
    }

    counts
}

fn simulate_for(counts: &mut [u64; 9], nb_turns: usize) {
    // To simulate the turns, instead of rotating arrays to
    // have:
    // - counts[0] => fishes at age 0,
    // - counts[1] => fishes at age 1,
    // We consider the counts as a ring, and rotate the index:
    // - counts[idx + 0] => fishes at age 0,
    // - counts[idx + 1] => fishes at age 1,
    // On a new turn, we now simply have to:
    // - add counts[idx + 0] to counts[idx + 7] (ie next index + 1)
    // - increment the index (old index becomes new_index + 8)
    let mut idx = 0;
    for _ in 0..nb_turns {
        counts[(idx + 7) % 9] += counts[idx];
        idx = (idx + 1) % 9;
    }
}
