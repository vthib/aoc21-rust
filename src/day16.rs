pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut input = InputStream::new(input);

    let packet = input.next_packet();

    let mut added_versions = 0;
    add_versions(&mut added_versions, &packet);
    added_versions
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let mut input = InputStream::new(input);

    let packet = input.next_packet();

    compute(&packet)
}

fn add_versions(acc: &mut u64, packet: &Packet) {
    *acc += packet.version;
    match &packet.typ {
        PacketType::Literal(_) => (),
        PacketType::Sum(ps)
        | PacketType::Product(ps)
        | PacketType::Minimum(ps)
        | PacketType::Maximum(ps) => {
            for p in ps {
                add_versions(acc, p);
            }
        }
        PacketType::GreaterThan(a, b) | PacketType::LessThan(a, b) | PacketType::EqualTo(a, b) => {
            add_versions(acc, a);
            add_versions(acc, b);
        }
    }
}

fn compute(packet: &Packet) -> u64 {
    match &packet.typ {
        PacketType::Literal(n) => *n,
        PacketType::Sum(ps) => ps.iter().map(compute).sum(),
        PacketType::Product(ps) => ps.iter().map(compute).product(),
        PacketType::Minimum(ps) => ps.iter().map(compute).min().unwrap(),
        PacketType::Maximum(ps) => ps.iter().map(compute).max().unwrap(),
        PacketType::GreaterThan(a, b) => {
            if compute(a) > compute(b) {
                1
            } else {
                0
            }
        }
        PacketType::LessThan(a, b) => {
            if compute(a) < compute(b) {
                1
            } else {
                0
            }
        }
        PacketType::EqualTo(a, b) => {
            if compute(a) == compute(b) {
                1
            } else {
                0
            }
        }
    }
}
struct Packet {
    version: u64,
    typ: PacketType,
}

enum PacketType {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Box<Packet>, Box<Packet>),
    LessThan(Box<Packet>, Box<Packet>),
    EqualTo(Box<Packet>, Box<Packet>),
}

#[derive(Debug)]
struct InputStream<'a> {
    input: std::str::Bytes<'a>,
    current_byte: u8,
    current_bit_shift: u8,
    nb_bits_eaten: u64,
}

impl<'a> InputStream<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.bytes(),
            current_byte: 0,
            current_bit_shift: 0,
            nb_bits_eaten: 0,
        }
    }

    fn get(&mut self, n: usize) -> u64 {
        let mut res = 0;

        for _ in 0..n {
            res = (res << 1) | self.get_bit()
        }
        res
    }

    fn get_bit(&mut self) -> u64 {
        if self.current_bit_shift == 0 {
            let a = hex_byte_to_u8(self.input.next().unwrap());
            let b = hex_byte_to_u8(self.input.next().unwrap());
            self.current_byte = (a << 4) | b;
            self.current_bit_shift = 7;
        } else {
            self.current_bit_shift -= 1;
        }
        self.nb_bits_eaten += 1;
        if self.current_byte & (1 << self.current_bit_shift) != 0 {
            1
        } else {
            0
        }
    }

    fn next_packet(&mut self) -> Packet {
        let version = self.get(3);
        let type_id = self.get(3);
        match type_id {
            4 => {
                let mut lit = 0u64;
                loop {
                    let n = self.get(5);
                    lit = (lit << 4) + (n & 0x0F);
                    if (n & 0x10) == 0 {
                        return Packet {
                            version,
                            typ: PacketType::Literal(lit),
                        };
                    }
                }
            }
            0 => Packet {
                version,
                typ: PacketType::Sum(self.parse_subpackets()),
            },
            1 => Packet {
                version,
                typ: PacketType::Product(self.parse_subpackets()),
            },
            2 => Packet {
                version,
                typ: PacketType::Minimum(self.parse_subpackets()),
            },
            3 => Packet {
                version,
                typ: PacketType::Maximum(self.parse_subpackets()),
            },
            5 => {
                let (a, b) = self.parse_pairpackets();
                Packet {
                    version,
                    typ: PacketType::GreaterThan(Box::new(a), Box::new(b)),
                }
            }
            6 => {
                let (a, b) = self.parse_pairpackets();
                Packet {
                    version,
                    typ: PacketType::LessThan(Box::new(a), Box::new(b)),
                }
            }
            7 => {
                let (a, b) = self.parse_pairpackets();
                Packet {
                    version,
                    typ: PacketType::EqualTo(Box::new(a), Box::new(b)),
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_subpackets(&mut self) -> Vec<Packet> {
        let length_type_id = self.get(1);
        if length_type_id == 0 {
            let length = self.get(15);
            let nb_bits_eaten_to_reach = self.nb_bits_eaten + length;
            let mut subpackets = Vec::new();
            loop {
                subpackets.push(self.next_packet());
                if self.nb_bits_eaten >= nb_bits_eaten_to_reach {
                    break;
                }
            }
            subpackets
        } else {
            let nb_subpackets = self.get(11);
            let mut subpackets = Vec::with_capacity(nb_subpackets as usize);
            for _ in 0..nb_subpackets {
                subpackets.push(self.next_packet());
            }
            subpackets
        }
    }

    fn parse_pairpackets(&mut self) -> (Packet, Packet) {
        let length_type_id = self.get(1);

        let len = self.get(if length_type_id == 0 { 15 } else { 11 });
        let nb_bits_before = self.nb_bits_eaten;
        let a = self.next_packet();
        let b = self.next_packet();

        if length_type_id == 0 {
            assert_eq!(self.nb_bits_eaten, nb_bits_before + len);
        } else {
            assert_eq!(len, 2);
        }
        (a, b)
    }
}

fn hex_byte_to_u8(c: u8) -> u8 {
    if c >= b'A' {
        c - b'A' + 10
    } else {
        c - b'0'
    }
}
