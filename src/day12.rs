use std::collections::HashMap;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let nodes = parse_input(input);

    let start = nodes.get("start").unwrap();
    let mut walked_nodes = vec!["start".to_owned()];
    let mut small_caves_visited = HashMap::new();
    small_caves_visited.insert("start".to_owned(), 2);

    compute_nb_walks(
        &mut walked_nodes,
        &nodes,
        start,
        &mut small_caves_visited,
        true,
    )
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let nodes = parse_input(input);

    let start = nodes.get("start").unwrap();
    let mut walked_nodes = vec!["start".to_owned()];
    let mut small_caves_visited = HashMap::new();
    small_caves_visited.insert("start".to_owned(), 2);

    compute_nb_walks(
        &mut walked_nodes,
        &nodes,
        start,
        &mut small_caves_visited,
        false,
    )
}

fn compute_nb_walks(
    walked_nodes: &mut Vec<String>,
    nodes: &HashMap<String, Node>,
    current_node: &Node,
    small_caves_visited: &mut HashMap<String, u32>,
    has_visited_small_twice: bool,
) -> u32 {
    let mut res = 0;

    if current_node.end {
        // println!("valid path: {}", walked_nodes.join("-"));
        return 1;
    }

    fn visit_node(
        walked_nodes: &mut Vec<String>,
        nodes: &HashMap<String, Node>,
        next_node_name: &str,
        next_node: &Node,
        small_caves_visited: &mut HashMap<String, u32>,
        has_visited_small_twice: bool,
    ) -> u32 {
        let current_walked_len = walked_nodes.len();

        walked_nodes.push(next_node_name.to_owned());
        let res = compute_nb_walks(
            walked_nodes,
            nodes,
            next_node,
            small_caves_visited,
            has_visited_small_twice,
        );
        walked_nodes.truncate(current_walked_len);
        res
    }

    for next_node_name in &current_node.to {
        let next_node = nodes.get(next_node_name).unwrap();

        if !next_node.small {
            res += visit_node(
                walked_nodes,
                nodes,
                next_node_name,
                next_node,
                small_caves_visited,
                has_visited_small_twice,
            );
            continue;
        }

        let nb_visited = small_caves_visited
            .entry(next_node_name.to_owned())
            .or_insert(0);
        if *nb_visited >= 2 {
            continue;
        }
        *nb_visited += 1;
        let next_visited_twice = *nb_visited >= 2;

        if !next_visited_twice || !has_visited_small_twice {
            res += visit_node(
                walked_nodes,
                nodes,
                next_node_name,
                next_node,
                small_caves_visited,
                if next_visited_twice {
                    true
                } else {
                    has_visited_small_twice
                },
            );
        }

        *small_caves_visited.get_mut(next_node_name).unwrap() -= 1;
    }
    res
}

struct Node {
    to: Vec<String>,
    end: bool,
    small: bool,
}

fn parse_input(input: &str) -> HashMap<String, Node> {
    let mut res = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split('-');
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();
        assert!(iter.next().is_none());

        // Extra alloc & dump if entry already exists, but meh
        res.entry(from.to_owned())
            .or_insert_with(|| Node::new(from))
            .add_to(to);
        res.entry(to.to_owned())
            .or_insert_with(|| Node::new(to))
            .add_to(from);
    }

    res
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            to: Vec::with_capacity(5),
            end: name == "end",
            small: name.chars().all(|c| c.is_ascii_lowercase()),
        }
    }

    fn add_to(&mut self, to: &str) {
        self.to.push(to.to_owned());
    }
}
