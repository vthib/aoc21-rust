pub fn part1(input: &str) -> impl std::fmt::Display {
    let entries = parse_input(input);

    let mut count = 0;
    for entry in entries {
        for signal in entry.iter().skip(10) {
            match signal.0.count_ones() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            }
        }
    }
    count
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    parse_input(input)
        .iter()
        .map(|entry| deduce_entry(entry))
        .sum::<u32>()
}
fn parse_input(input: &str) -> Vec<Vec<Signals>> {
    input
        .lines()
        .map(|line| {
            let mut elems = line.split(" | ");
            let left = elems.next().unwrap();
            let right = elems.next().unwrap();

            let mut left: Vec<_> = left.split(' ').map(|v| Signals::from_str(v)).collect();
            assert!(left.len() == 10);
            let mut right: Vec<_> = right.split(' ').map(|v| Signals::from_str(v)).collect();
            assert!(right.len() == 4);
            left.append(&mut right);
            left
        })
        .collect()
}

// Set of signals being activated. This is a bitmap
//
//   aaaa
//  b    c
//  b    c
//   dddd
//  e    f
//  e    f
//   gggg
//
//  eg, adeg would give 0x01011001.
struct Signals(u8);

impl Signals {
    fn from_str(input: &str) -> Self {
        let mut res = 0;

        for c in input.chars() {
            match c {
                'a'..='g' => res |= 1 << (c as u8 - b'a'),
                _ => unreachable!(),
            }
        }
        Self(res)
    }
}

#[allow(clippy::many_single_char_names)]
fn deduce_entry(signals: &[Signals]) -> u32 {
    let mut maps = [0; 10];

    // find 1, 4, 7 and 8
    for input in signals {
        match input.0.count_ones() {
            2 => maps[1] = input.0,
            3 => maps[7] = input.0,
            4 => maps[4] = input.0,
            7 => maps[8] = input.0,
            _ => (),
        }
    }

    // digits with 6 signals:
    // - '9' is only one containing '4'
    // - '0' is only one containing '1' and not '4'
    // - '6' is the other one
    for input in signals {
        if input.0.count_ones() == 6 {
            if input.0 & maps[4] == maps[4] {
                maps[9] = input.0;
            } else if input.0 & maps[1] == maps[1] {
                maps[0] = input.0;
            } else {
                maps[6] = input.0;
            }
        }
    }

    // from there, we can deduce all signals
    let a = maps[7] & !maps[1];
    let d = !maps[0] & 0x7F;
    let b = maps[4] & (!(maps[1] | d) & 0x7F);
    let c = maps[1] & !maps[6];
    let e = !maps[9] & 0x7F;
    let f = maps[1] ^ c;
    let g = (!maps[4] & 0x7F) ^ a ^ e;
    assert!(a.count_ones() == 1);
    assert!(b.count_ones() == 1);
    assert!(c.count_ones() == 1);
    assert!(d.count_ones() == 1);
    assert!(e.count_ones() == 1);
    assert!(f.count_ones() == 1);
    assert!(g.count_ones() == 1);

    // compute last missing digits
    maps[2] = a ^ c ^ d ^ e ^ g;
    maps[3] = a ^ c ^ d ^ f ^ g;
    maps[5] = a ^ b ^ d ^ f ^ g;

    // compute output value
    let mut res: u32 = 0;
    for signal in signals.iter().skip(10) {
        for (i, m) in maps.iter().enumerate() {
            if *m == signal.0 {
                res = res * 10 + i as u32;
                break;
            }
        }
    }
    res
}
