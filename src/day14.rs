use std::collections::HashMap;

pub fn part1(input: &str) -> impl std::fmt::Display {
    inner(input, 10)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    inner(input, 40)
}

fn inner(input: &str, nb_iterations: u32) -> impl std::fmt::Display {
    let (polymer, recipes) = parse_input(input);

    // Do not compute the full polymer on every iteration.
    // Instead, count the pairs, and increase those counts.
    let mut pairs = HashMap::new();
    for elems in polymer.windows(2) {
        *pairs.entry((elems[0], elems[1])).or_insert(0u64) += 1;
    }

    for _ in 0..nb_iterations {
        pairs = transform_polymer(&pairs, &recipes);
    }

    // group the elems per occurences
    let mut occurences = HashMap::new();
    for ((a, b), count) in pairs {
        *occurences.entry(a).or_insert(0u64) += count;
        *occurences.entry(b).or_insert(0u64) += count;
    }
    // We counted occurences per chars, but every char is counted twice: as X. and as .X,
    // except for the first and the last character, counted once. increase those by 1
    // to double their count and fix this issue.
    *occurences.get_mut(&polymer[0]).unwrap() += 1;
    *occurences.get_mut(&polymer[polymer.len() - 1]).unwrap() += 1;

    let min = occurences.iter().min_by_key(|(_, v)| **v).unwrap();
    let max = occurences.iter().max_by_key(|(_, v)| **v).unwrap();

    (max.1 - min.1) / 2
}
fn transform_polymer(
    pairs: &HashMap<(char, char), u64>,
    recipes: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    let mut new_pairs = HashMap::new();
    for ((a, b), count) in pairs {
        let new_elem = recipes.get(&(*a, *b)).unwrap();

        *new_pairs.entry((*a, *new_elem)).or_insert(0u64) += count;
        *new_pairs.entry((*new_elem, *b)).or_insert(0u64) += count;
    }
    new_pairs
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut elems = input.split("\n\n");
    let polymer = elems.next().unwrap();
    let recipes = elems.next().unwrap();
    assert!(elems.next().is_none());

    (
        polymer.chars().collect(),
        recipes
            .lines()
            .map(|line| {
                assert_eq!(&line[2..6], " -> ");
                let mut line = line.chars();
                let first = line.next().unwrap();
                let second = line.next().unwrap();
                let new_elem = line.nth(4).unwrap();
                assert!(line.next().is_none());
                ((first, second), new_elem)
            })
            .collect(),
    )
}
