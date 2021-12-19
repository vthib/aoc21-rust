use std::cmp::Ordering;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let scanners = parse_input(input);
    let scanners = fix_scanner_positions(scanners);

    let mut all_beacons: Vec<_> = scanners.iter().flat_map(|s| s.beacons.clone()).collect();
    all_beacons.sort_unstable();
    all_beacons.dedup();
    all_beacons.len()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let scanners = parse_input(input);
    let scanners = fix_scanner_positions(scanners);

    let mut max_manhattan_distance = 0;
    for i in 0..scanners.len() {
        for j in (i + 1)..scanners.len() {
            max_manhattan_distance = std::cmp::max(
                max_manhattan_distance,
                scanners[i].manhattan_distance(&scanners[j]),
            )
        }
    }
    max_manhattan_distance
}

fn fix_scanner_positions(mut scanners: Vec<Scanner>) -> Vec<Scanner> {
    let mut fixed_scanners = Vec::with_capacity(scanners.len());
    fixed_scanners.push(scanners.remove(0));

    'next_scanner: while !scanners.is_empty() {
        for (i, s) in scanners.iter_mut().enumerate() {
            for f in &fixed_scanners {
                if f.match_beacons(s) {
                    println!("match scanner {} with {}", &s.name, &f.name);
                    fixed_scanners.push(scanners.remove(i));
                    continue 'next_scanner;
                }
            }
        }
    }

    assert!(!fixed_scanners.is_empty());
    fixed_scanners
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn rotate_rx(&mut self, angle: i32) {
        match angle {
            90 => {
                let saved = self.y;
                self.y = -self.z;
                self.z = saved;
            }
            _ => unreachable!(),
        }
    }

    fn rotate_ry(&mut self, angle: i32) {
        match angle {
            90 => {
                let saved = self.x;
                self.x = self.z;
                self.z = -saved;
            }
            _ => unreachable!(),
        }
    }

    fn rotate_rz(&mut self, angle: i32) {
        match angle {
            90 => {
                let saved = self.x;
                self.x = -self.y;
                self.y = saved;
            }
            180 => {
                self.x = -self.x;
                self.y = -self.y;
            }
            _ => unreachable!(),
        }
    }

    fn rotate(&mut self, rx: i32, ry: i32, rz: i32) {
        if rx != 0 {
            self.rotate_rx(rx);
        }
        if ry != 0 {
            self.rotate_ry(rx);
        }
        if rz != 0 {
            self.rotate_rz(rx);
        }
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Clone)]
struct Scanner {
    name: String,
    beacons: Vec<Coord>,
    position: (i32, i32, i32),
}

impl Scanner {
    fn new(name: String, mut beacons: Vec<Coord>) -> Self {
        // Store beacons sorted.
        beacons.sort_unstable();
        Self {
            name,
            beacons,
            position: (0, 0, 0),
        }
    }

    fn manhattan_distance(&self, other: &Scanner) -> i32 {
        (self.position.0 - other.position.0).abs()
            + (self.position.1 - other.position.1).abs()
            + (self.position.2 - other.position.2).abs()
    }

    fn match_beacons(&self, other: &mut Scanner) -> bool {
        if let Some(rel_pos_other) = self.try_match_beacons(other) {
            // Re orient the other beacons to match the relative coordinates of Self.
            println!(
                "position of {}: ({}, {}, {})",
                &other.name, rel_pos_other.0, rel_pos_other.1, rel_pos_other.2
            );
            for b in &mut other.beacons {
                b.x += rel_pos_other.0;
                b.y += rel_pos_other.1;
                b.z += rel_pos_other.2;
            }
            other.beacons.sort_unstable();
            other.position = rel_pos_other;
            true
        } else {
            false
        }
    }

    fn try_match_beacons(&self, other: &mut Scanner) -> Option<(i32, i32, i32)> {
        // Since orientation of the scanner is unknown, we need to try every
        // orientation possible for the other scanner.
        // Since we mutate the beacons, each new orientation depends on the
        // previous one.
        const ROTATIONS: [(i32, i32, i32); 24] = [
            (0, 0, 0),
            (90, 0, 0),
            (90, 0, 0),
            (90, 0, 0),
            (90, 90, 0),
            (90, 0, 0),
            (90, 0, 0),
            (90, 0, 0),
            (90, 90, 0),
            (90, 0, 0),
            (90, 0, 0),
            (90, 0, 0),
            (90, 90, 0),
            (90, 0, 0),
            (90, 0, 0),
            (90, 0, 0),
            (90, 90, 90),
            (90, 0, 0),
            (90, 0, 0),
            (90, 0, 0),
            (90, 0, 180),
            (90, 0, 0),
            (90, 0, 0),
            (90, 0, 0),
        ];

        for rotation in &ROTATIONS {
            for b in &mut other.beacons {
                b.rotate(rotation.0, rotation.1, rotation.2);
            }
            other.beacons.sort_unstable();
            if let Some(res) = self.try_match_orientation(other) {
                return Some(res);
            }
        }
        None
    }

    // Try to match beacons between two scanners. `other` has been fixed on
    // one orientation.
    // If successful, it returns the position of `other` relative to `self`.
    fn try_match_orientation(&self, other: &Scanner) -> Option<(i32, i32, i32)> {
        // To match beacons, match a first pair, then count how many others are
        // matched accordingly.
        let mut res = Vec::with_capacity(12);

        for i0 in 0..(self.beacons.len() - 11) {
            for j0 in 0..(other.beacons.len() - 11) {
                let sb0 = self.beacons[i0];
                let ob0 = other.beacons[j0];

                res.push((sb0, ob0));
                let diff = (sb0.x - ob0.x, sb0.y - ob0.y, sb0.z - ob0.z);

                let mut i = i0 + 1;
                let mut j = j0 + 1;
                loop {
                    if i >= self.beacons.len() || j >= other.beacons.len() {
                        break;
                    }
                    let mut ob = other.beacons[j];
                    ob.x += diff.0;
                    ob.y += diff.1;
                    ob.z += diff.2;
                    match self.beacons[i].cmp(&ob) {
                        Ordering::Less => i += 1,
                        Ordering::Greater => j += 1,
                        Ordering::Equal => {
                            res.push((self.beacons[i], other.beacons[j]));
                            i += 1;
                            j += 1;
                        }
                    }
                }
                if res.len() >= 12 {
                    return Some(diff);
                }
                res.clear();
            }
        }
        None
    }
}

fn parse_input(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .enumerate()
        .map(|(i, block)| {
            Scanner::new(
                i.to_string(),
                block
                    .lines()
                    .skip(1)
                    .map(|line| {
                        let mut coords = line.split(',');

                        Coord {
                            x: coords.next().unwrap().parse().unwrap(),
                            y: coords.next().unwrap().parse().unwrap(),
                            z: coords.next().unwrap().parse().unwrap(),
                        }
                    })
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn test_matching_beacons() {
        fn test(
            a: &[(i32, i32, i32)],
            b: &[(i32, i32, i32)],
            expected: (i32, i32, i32),
        ) {
            let a: Vec<_> = a
                .iter()
                .map(|v| Coord { x: v.0, y: v.1, z: v.2, })
                .collect();
            let b: Vec<_> = b
                .iter()
                .map(|v| Coord { x: v.0, y: v.1, z: v.2, })
                .collect();

            let s1 = Scanner::new("0".to_owned(), a);
            let mut s2 = Scanner::new("1".to_owned(), b);
            let res = s1.try_match_beacons(&mut s2).unwrap();
            assert_eq!(res, expected);
        }

        test(
            &[
                (404, -588, -901), (528, -643, 409), (-838, 591, 734), (390, -675, -793),
                (-537, -823, -458), (-485, -357, 347), (-345, -311, 381), (-661, -816, -575),
                (-876, 649, 763), (-618, -824, -621), (553, 345, -567), (474, 580, 667),
                (-447, -329, 318), (-584, 868, -557), (544, -627, -890), (564, 392, -477),
                (455, 729, 728), (-892, 524, 684), (-689, 845, -530), (423, -701, 434),
                (7, -33, -71), (630, 319, -379), (443, 580, 662), (-789, 900, -551),
                (459, -707, 401),
            ],
            &[
                (686, 422, 578), (605, 423, 415), (515, 917, -361), (-336, 658, 858),
                (95, 138, 22), (-476, 619, 847), (-340, -569, -846), (567, -361, 727),
                (-460, 603, -452), (669, -402, 600), (729, 430, 532), (-500, -761, 534),
                (-322, 571, 750), (-466, -666, -811), (-429, -592, 574), (-355, 545, -477),
                (703, -491, -529), (-328, -685, 520), (413, 935, -424), (-391, 539, -444),
                (586, -435, 557), (-364, -763, -893), (807, -499, -711), (755, -354, -619),
                (553, 889, -390),
            ],
            (68,-1246,-43),
        );
        test(
            &[
                (-686,  422, -578), (-605,  423, -415), (-515,  917,  361), ( 336,  658, -858),
                ( -95,  138,  -22), ( 476,  619, -847), ( 340, -569,  846), (-567, -361, -727),
                ( 460,  603,  452), (-669, -402, -600), (-729,  430, -532), ( 500, -761, -534),
                ( 322,  571, -750), ( 466, -666,  811), ( 429, -592, -574), ( 355,  545,  477),
                (-703, -491,  529), ( 328, -685, -520), (-413,  935,  424), ( 391,  539,  444),
                (-586, -435, -557), ( 364, -763,  893), (-807, -499,  711), (-755, -354,  619),
                (-553,  889,  390),
            ],
            &[
                ( 727,  592,  562), (-293, -554,  779), ( 441,  611, -461), (-714,  465, -776),
                (-743,  427, -804), (-660, -479, -426), ( 832, -632,  460), ( 927, -485, -438),
                ( 408,  393, -506), ( 466,  436, -512), ( 110,   16,  151), (-258, -428,  682),
                (-393,  719,  612), (-211, -452,  876), ( 808, -476, -593), (-575,  615,  604),
                (-485,  667,  467), (-680,  325, -822), (-627, -443, -432), ( 872, -547, -609),
                ( 833,  512,  582), ( 807,  604,  487), ( 839, -516,  451), ( 891, -625,  532),
                (-652, -548, -490), (  30, -46,   -14),
            ],
            (-88, 113, 1104)
        )
    }
}
