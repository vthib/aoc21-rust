use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::map,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

pub fn part1(input: &str) -> impl std::fmt::Display {
    parse_input(input)
        .into_iter()
        .reduce(|a, b| (a + b).reduce())
        .unwrap()
        .magnitude()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let input = parse_input(input);

    let mut max_magnitude = 0;
    let mut test_pair = |a: &Node, b: &Node| {
        let res: Node = a.clone() + b.clone();
        let res = res.reduce();
        let magnitude = res.magnitude();
        if magnitude > max_magnitude {
            max_magnitude = magnitude;
        }
    };
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            test_pair(&input[i], &input[j]);
            test_pair(&input[j], &input[i]);
        }
    }
    max_magnitude
}

#[derive(Clone, PartialEq, Debug)]
enum Node {
    Leaf(u32),
    Pair(Box<Node>, Box<Node>),
}

impl std::ops::Add<Node> for Node {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::Pair(Box::new(self), Box::new(rhs))
    }
}

impl Node {
    fn reduce(mut self) -> Self {
        loop {
            if self.explode(0).is_some() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
        self
    }

    fn explode(&mut self, depth: u8) -> Option<(u32, u32)> {
        match self {
            Self::Leaf(_) => None,
            Self::Pair(left, right) => {
                if let Some((l, r)) = match (&**left, &**right) {
                    (Self::Leaf(left), Self::Leaf(right)) if depth >= 4 => Some((*left, *right)),
                    _ => None,
                } {
                    *self = Self::Leaf(0);
                    Some((l, r))
                } else if let Some((left_val, right_val)) = left.explode(depth + 1) {
                    if right_val > 0 {
                        // Add a value to the left most node in the right branch
                        right.add_left_value(right_val);
                    }
                    Some((left_val, 0))
                } else if let Some((left_val, right_val)) = right.explode(depth + 1) {
                    if left_val > 0 {
                        // Add a value to the right most node in the left branch
                        left.add_right_value(left_val);
                    }
                    Some((0, right_val))
                } else {
                    None
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Node::Leaf(n) if *n >= 10 => {
                *self = Node::Pair(
                    Box::new(Node::Leaf(*n / 2)),
                    Box::new(Node::Leaf((*n + 1) / 2)),
                );
                true
            }
            Node::Leaf(_) => false,
            Node::Pair(left, right) => left.split() || right.split(),
        }
    }

    fn add_left_value(&mut self, val: u32) {
        match self {
            Node::Leaf(n) => *n += val,
            Node::Pair(left, _) => left.add_left_value(val),
        }
    }

    fn add_right_value(&mut self, val: u32) {
        match self {
            Node::Leaf(n) => *n += val,
            Node::Pair(_, right) => right.add_right_value(val),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Node::Leaf(n) => *n,
            Node::Pair(a, b) => a.magnitude() * 3 + b.magnitude() * 2,
        }
    }
}

impl std::str::FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_node(s)
            .finish()
            .map(|(_, node)| node)
            .map_err(|e| format!("nom error: {:?}", e))
    }
}
fn parse_node(input: &str) -> IResult<&str, Node> {
    alt((
        map(digit1, |s: &str| Node::Leaf(s.parse::<u32>().unwrap())),
        delimited(
            char('['),
            map(
                separated_pair(parse_node, char(','), parse_node),
                |(a, b)| a + b,
            ),
            char(']'),
        ),
    ))(input)
}

fn parse_input(input: &str) -> Vec<Node> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fn test_explode_once(before: &str, after: &str) {
            let mut root = before.parse::<Node>().unwrap();
            assert!(root.explode(0).is_some());

            let expected_root = after.parse::<Node>().unwrap();
            assert_eq!(root, expected_root);
        }

        fn test_reduction(before: &str, after: &str) {
            let mut root = before.parse::<Node>().unwrap();
            root = root.reduce();

            let expected_root = after.parse::<Node>().unwrap();
            assert_eq!(&root, &expected_root);
        }

        test_explode_once("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        test_explode_once("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        test_explode_once("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        test_explode_once(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        test_explode_once(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );

        test_reduction(
            "[[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]],[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]]",
            "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]"
        )
    }
}
