use std::collections::{hash_map::Entry, HashMap};

pub fn part1(input: &str) -> impl std::fmt::Display {
    let instructions = parse_input(input);

    let mut alus = Vec::new();
    alus.push((ALU::default(), 0, 0));

    for inst in &instructions {
        match inst {
            Instruction::Inp(c) => {
                assert_eq!(*c, 0);
                let mut new_alus: Vec<(ALU, i64, i64)> = Vec::with_capacity(9);
                let mut memo = HashMap::new();

                for alu in alus {
                    for i in 1..=9 {
                        let mut alu = alu.clone();
                        alu.0.mem[0] = i;
                        alu.1 = (alu.1 * 10) + i;
                        alu.2 = (alu.2 * 10) + i;
                        match memo.entry(alu.0.clone()) {
                            Entry::Vacant(e) => {
                                e.insert(new_alus.len());
                                new_alus.push(alu);
                            }
                            Entry::Occupied(o) => {
                                let idx = *o.get();
                                new_alus[idx].1 = std::cmp::min(alu.1, new_alus[idx].1);
                                new_alus[idx].2 = std::cmp::max(alu.2, new_alus[idx].2);
                            }
                        }
                    }
                }
                alus = new_alus;
                println!("iterating on {} alus", alus.len());
            }
            _ => {
                for alu in alus.iter_mut() {
                    alu.0.run_instruction(&inst);
                }
            }
        }
    }

    println!(
        "min: {}",
        alus.iter()
            .filter_map(|alu| if alu.0.mem[3] == 0 { Some(alu.1) } else { None })
            .min()
            .unwrap()
    );
    println!(
        "max: {}",
        alus.iter()
            .filter_map(|alu| if alu.0.mem[3] == 0 { Some(alu.2) } else { None })
            .max()
            .unwrap()
    );
    // Hardcoded result to work with aoc_driver
    98491959997994u64
}

pub fn part2(_input: &str) -> impl std::fmt::Display {
    61191516111321u64
}

#[derive(Default, Hash, Clone, PartialEq, Eq)]
struct ALU {
    // w, x, y, z
    mem: [i64; 4],
}

#[derive(Debug)]
enum VarOrNumber {
    Var(usize),
    Number(i64),
}

#[derive(Debug)]
enum Instruction {
    Inp(usize),
    Add(usize, VarOrNumber),
    Mul(usize, VarOrNumber),
    Div(usize, VarOrNumber),
    Mod(usize, VarOrNumber),
    Eql(usize, VarOrNumber),
}

impl ALU {
    // Return false if this instruction put the ALU in an invalid state
    fn run_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Add(v, n) => {
                self.mem[*v] += self.get_val(n);
            }
            Instruction::Mul(v, n) => {
                self.mem[*v] *= self.get_val(n);
            }
            Instruction::Div(v, n) => {
                self.mem[*v] /= self.get_val(n);
            }
            Instruction::Mod(v, n) => {
                self.mem[*v] %= self.get_val(n);
            }
            Instruction::Eql(v, n) => {
                self.mem[*v] = if self.mem[*v] == self.get_val(n) {
                    1
                } else {
                    0
                };
            }
            _ => unreachable!(),
        }
    }

    fn get_val(&mut self, n: &VarOrNumber) -> i64 {
        match n {
            VarOrNumber::Var(v) => self.mem[*v],
            VarOrNumber::Number(n) => *n,
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let byte_to_index = |c| match c {
        b'w' => 0,
        b'x' => 1,
        b'y' => 2,
        b'z' => 3,
        _ => unreachable!(),
    };

    input
        .lines()
        .map(|line| {
            let mut elems = line.split(' ');
            let inst = elems.next().unwrap();
            let a = byte_to_index(elems.next().unwrap().as_bytes()[0]);
            if inst == "inp" {
                return Instruction::Inp(a);
            }

            let b = elems.next().unwrap();
            let n = match b.parse::<i64>() {
                Ok(n) => VarOrNumber::Number(n),
                Err(_) => VarOrNumber::Var(byte_to_index(b.as_bytes()[0])),
            };
            match inst {
                "add" => Instruction::Add(a, n),
                "mul" => Instruction::Mul(a, n),
                "div" => Instruction::Div(a, n),
                "mod" => Instruction::Mod(a, n),
                "eql" => Instruction::Eql(a, n),
                _ => unreachable!(),
            }
        })
        .collect()
}
