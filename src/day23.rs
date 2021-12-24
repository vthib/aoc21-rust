use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn part1(input: &str) -> impl std::fmt::Display {
    let situation = parse_input(input);

    dijkstra(situation)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let s = parse_input(input);

    let situation = Situation {
        rooms: [
            [
                s.rooms[0][0],
                Some(Amphipod::Desert),
                Some(Amphipod::Desert),
                s.rooms[0][1],
            ],
            [
                s.rooms[1][0],
                Some(Amphipod::Copper),
                Some(Amphipod::Bronze),
                s.rooms[1][1],
            ],
            [
                s.rooms[2][0],
                Some(Amphipod::Bronze),
                Some(Amphipod::Amber),
                s.rooms[2][1],
            ],
            [
                s.rooms[3][0],
                Some(Amphipod::Amber),
                Some(Amphipod::Copper),
                s.rooms[3][1],
            ],
        ],
        hallway: [None; 7],
    };
    dijkstra(situation)
}

// Do Dijkstra's algorithm to find smallest cost to solution path
fn dijkstra<const N: usize>(situation: Situation<N>) -> u64 {
    let mut heap = DijkstraHeap {
        heap: BinaryHeap::new(),
        // current smallest cost to reach this situation from 0,0 to y*width + x
        costs: HashMap::new(),
    };

    heap.costs.insert(situation.clone(), 0);
    heap.heap
        .push(Reverse(SituationWithCost { cost: 0, situation }));

    // pop, compute distance to neighbor, repeat
    while let Some(situation) = heap.heap.pop() {
        let situation = situation.0;
        let s = &situation.situation;
        let cost = situation.cost;

        if s.is_valid() {
            return cost;
        }

        // We already have a better cost to this position, so ignore
        if let Some(existing_cost) = heap.costs.get(&s) {
            if cost > *existing_cost {
                continue;
            }
        }

        // From room to hallway
        for ridx in 0..4 {
            // index in room
            let index_in_room = match s.rooms[ridx].iter().position(|p| p.is_some()) {
                None => continue,
                Some(v) => v,
            };

            for hidx in s
                .hallway
                .iter()
                .enumerate()
                .filter_map(|(i, p)| p.is_none().then(|| i))
            {
                if !s.path_obstructed(hidx, ridx) {
                    let mut new_situation = situation.clone();
                    new_situation.do_move(hidx, ridx, index_in_room);
                    heap.push(new_situation);
                }
            }
        }

        // From hallway to room
        for (hidx, h) in s.hallway.iter().enumerate() {
            match h {
                Some(amphipod) => {
                    let room_idx = amphipod.room_idx();
                    if s.rooms[room_idx].iter().all(|p| match p {
                        None => true,
                        Some(v) => v == amphipod,
                    }) {
                        if !s.path_obstructed(hidx, room_idx) {
                            let mut new_situation = situation.clone();
                            new_situation.do_move(
                                hidx,
                                room_idx,
                                s.rooms[room_idx].iter().position(|p| p.is_none()).unwrap(),
                            );
                            heap.push(new_situation);
                        }
                    }
                }
                None => (),
            }
        }
    }
    unreachable!()
}

struct DijkstraHeap<const N: usize> {
    heap: BinaryHeap<Reverse<SituationWithCost<N>>>,
    costs: HashMap<Situation<N>, u64>,
}

impl<const N: usize> DijkstraHeap<N> {
    fn push(&mut self, elem: SituationWithCost<N>) {
        if elem.cost >= *self.costs.get(&elem.situation).unwrap_or(&u64::MAX) {
            return;
        }
        self.costs.insert(elem.situation.clone(), elem.cost);
        self.heap.push(Reverse(elem));
    }
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct SituationWithCost<const N: usize> {
    cost: u64,
    situation: Situation<N>,
}

impl<const N: usize> SituationWithCost<N> {
    fn do_move(&mut self, hidx: usize, ridx: usize, index_in_room: usize) {
        let amphipod = match (
            self.situation.hallway[hidx],
            self.situation.rooms[ridx][index_in_room],
        ) {
            (Some(v), None) | (None, Some(v)) => v,
            _ => unreachable!(),
        };
        self.cost += (distance(hidx, ridx) + (index_in_room as u64)) * amphipod.move_cost();
        std::mem::swap(
            &mut self.situation.hallway[hidx],
            &mut self.situation.rooms[ridx][index_in_room],
        );
    }
}

#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
struct Situation<const N: usize> {
    rooms: [[Option<Amphipod>; N]; 4],
    hallway: [Option<Amphipod>; 7],
}

impl<const N: usize> Situation<N> {
    fn is_valid(&self) -> bool {
        self.rooms[0].iter().all(|p| *p == Some(Amphipod::Amber))
            && self.rooms[1].iter().all(|p| *p == Some(Amphipod::Bronze))
            && self.rooms[2].iter().all(|p| *p == Some(Amphipod::Copper))
            && self.rooms[3].iter().all(|p| *p == Some(Amphipod::Desert))
    }

    fn path_obstructed(&self, hidx: usize, room_idx: usize) -> bool {
        if hidx <= room_idx + 1 {
            self.hallway[(hidx + 1)..=(room_idx + 1)]
                .iter()
                .any(|p| p.is_some())
        } else {
            self.hallway[(room_idx + 2)..hidx]
                .iter()
                .any(|p| p.is_some())
        }
    }
}

fn distance(hidx: usize, room_idx: usize) -> u64 {
    //
    // hallway:  01 2 3 4 56
    // room:       0 1 2 3
    //
    //     | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
    // | 0 | 3 | 2 | 2 | 4 | 6 | 8 | 9 |
    // | 1 | 5 | 4 | 2 | 2 | 4 | 6 | 7 |
    // | 2 | 7 | 6 | 4 | 2 | 2 | 4 | 5 |
    // | 3 | 9 | 8 | 6 | 4 | 2 | 2 | 3 |
    match (hidx, room_idx) {
        (1, 0) | (2, 0) | (2, 1) | (3, 1) | (3, 2) | (4, 2) | (4, 3) | (5, 3) => 2,
        (1, 1) | (2, 2) | (3, 0) | (3, 3) | (4, 1) | (5, 2) => 4,
        (1, 2) | (2, 3) | (4, 0) | (5, 1) => 6,
        (1, 3) | (5, 0) => 8,
        (0, 0) | (6, 3) => 3,
        (0, 1) | (6, 2) => 5,
        (0, 2) | (6, 1) => 7,
        (0, 3) | (6, 0) => 9,
        _ => unreachable!(),
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn new(typ: u8) -> Self {
        match typ {
            b'A' => Self::Amber,
            b'B' => Self::Bronze,
            b'C' => Self::Copper,
            b'D' => Self::Desert,
            _ => unreachable!(),
        }
    }

    fn move_cost(&self) -> u64 {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }

    fn room_idx(&self) -> usize {
        match self {
            Self::Amber => 0,
            Self::Bronze => 1,
            Self::Copper => 2,
            Self::Desert => 3,
        }
    }
}

fn parse_input(input: &str) -> Situation<2> {
    // Input looks like:
    //
    // #############
    // #...........#
    // ###B#C#B#D###
    //   #A#D#C#A#
    //   #########
    let mut lines = input.lines().skip(2);
    let mut l1 = lines.next().unwrap().bytes();
    let mut l2 = lines.next().unwrap().bytes();

    let amphipod11 = Amphipod::new(l1.nth(3).unwrap());
    let amphipod12 = Amphipod::new(l1.nth(1).unwrap());
    let amphipod13 = Amphipod::new(l1.nth(1).unwrap());
    let amphipod14 = Amphipod::new(l1.nth(1).unwrap());
    let amphipod21 = Amphipod::new(l2.nth(3).unwrap());
    let amphipod22 = Amphipod::new(l2.nth(1).unwrap());
    let amphipod23 = Amphipod::new(l2.nth(1).unwrap());
    let amphipod24 = Amphipod::new(l2.nth(1).unwrap());

    Situation {
        rooms: [
            [Some(amphipod11), Some(amphipod21)],
            [Some(amphipod12), Some(amphipod22)],
            [Some(amphipod13), Some(amphipod23)],
            [Some(amphipod14), Some(amphipod24)],
        ],
        hallway: [None; 7],
    }
}
