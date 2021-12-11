pub fn part1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .filter_map(|line| complete_line(line).err())
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut scores: Vec<_> = input
        .lines()
        .filter_map(|line| complete_line(line).ok())
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn complete_line(line: &str) -> Result<u64, char> {
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            ')' | ']' | '}' | '>' => {
                if stack.pop() != Some(c) {
                    return Err(c);
                }
            }
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            _ => unreachable!(),
        }
    }

    // Compute the score by completing the line with missing closing elements
    Ok(stack.iter().rev().fold(0, |acc, c| {
        acc * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            }
    }))
}
