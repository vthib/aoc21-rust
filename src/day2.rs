pub fn part1(input: &str) -> impl std::fmt::Display {
    let movements = parse_input(input);

    let mut x = 0;
    let mut y = 0;
    for mv in movements {
        match mv {
            Movement::Forward(value) => x += value,
            Movement::Down(value) => y += value,
            Movement::Up(value) => y -= value,
        }
    }
    x * y
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let movements = parse_input(input);

    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;
    for mv in movements {
        match mv {
            Movement::Forward(value) => {
                x += value;
                depth += aim * value;
            }
            Movement::Down(value) => aim += value,
            Movement::Up(value) => aim -= value,
        }
    }
    x * depth
}

fn parse_input(input: &str) -> Vec<Movement> {
    input.lines().flat_map(|v| v.parse()).collect()
}

enum Movement {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl std::str::FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elems = s.split(' ');

        let dir = elems.next().unwrap();
        let value = elems.next().unwrap().parse::<u32>().unwrap();
        assert!(elems.next().is_none());

        Ok(match dir {
            "forward" => Self::Forward(value),
            "down" => Self::Down(value),
            "up" => Self::Up(value),
            _ => unreachable!(),
        })
    }
}
