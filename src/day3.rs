use std::cmp::Ordering;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let (values, max_length) = parse_input(input);

    let mut gamma_rate: u64 = 0;
    for i in (0..max_length).rev() {
        gamma_rate <<= 1;
        let cmpres = compare_bit_counts(&values, i);
        if cmpres == Ordering::Less {
            gamma_rate |= 1;
        }
    }
    let epsilon_rate = !gamma_rate & ((1 << max_length) - 1);
    gamma_rate * epsilon_rate
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let (values, max_length) = parse_input(input);

    let oxy_gen_rating =
        apply_bit_criteria(values.clone(), max_length, |bit, cmpres| match cmpres {
            Ordering::Less | Ordering::Equal => bit != 0,
            Ordering::Greater => bit == 0,
        });
    let co2_scrub_rating = apply_bit_criteria(values, max_length, |bit, cmpres| match cmpres {
        Ordering::Less | Ordering::Equal => bit == 0,
        Ordering::Greater => bit != 0,
    });
    oxy_gen_rating * co2_scrub_rating
}

fn apply_bit_criteria<F>(mut values: Vec<u64>, max_length: usize, cond: F) -> u64
where
    F: Fn(u64, Ordering) -> bool,
{
    for i in (0..max_length).rev() {
        let cmp_res = compare_bit_counts(&values, i);
        values = values
            .into_iter()
            .filter(|v| cond(*v & (1 << i), cmp_res))
            .collect();
        if values.len() == 1 {
            return values[0];
        }
    }
    unreachable!();
}

fn parse_input(input: &str) -> (Vec<u64>, usize) {
    let values = input.lines().map(|v| bin_string_to_u64(v)).collect();
    let max_length = input.lines().map(|v| v.len()).max().unwrap();
    (values, max_length)
}

fn bin_string_to_u64(input: &str) -> u64 {
    let mut v = 0;

    for c in input.chars() {
        v <<= 1;
        match c {
            '0' => (),
            '1' => v |= 1,
            _ => unreachable!(),
        };
    }
    v
}

fn compare_bit_counts(values: &[u64], bit_pos: usize) -> Ordering {
    let counts = values.iter().fold((0, 0), |(nb_0, nb_1), v| {
        if v & (1 << bit_pos) != 0 {
            (nb_0, nb_1 + 1)
        } else {
            (nb_0 + 1, nb_1)
        }
    });
    counts.0.cmp(&counts.1)
}
