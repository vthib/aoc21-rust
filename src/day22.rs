pub fn part1(input: &str) -> impl std::fmt::Display {
    let reboot_steps = parse_input(input);
    let mut reactor = Reactor::default();

    for step in reboot_steps.into_iter() {
        if step.cuboid.x.0 < -50 || step.cuboid.x.1 > 50 ||
           step.cuboid.y.0 < -50 || step.cuboid.y.1 > 50 ||
           step.cuboid.z.0 < -50 || step.cuboid.z.1 > 50 {
            continue;
        }

        reactor.apply_reboot_step(step);
    }

    reactor.count_activated_cuboids()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let reboot_steps = parse_input(input);
    let mut reactor = Reactor::default();

    for step in reboot_steps.into_iter() {
        reactor.apply_reboot_step(step);
    }

    reactor.count_activated_cuboids()
}

#[derive(Debug)]
struct Cuboid {
    // All of those are including range
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

// Compute intersection between [a0; a1] and [b0; b1]
fn intersect_one_axis(a: (i32, i32), b: (i32, i32)) -> Option<(i32, i32)> {
    if a.1 < b.0 || b.1 < a.0 {
        None
    } else if a.1 >= b.1 && a.0 <= b.0 {
        Some(b)
    } else if b.1 >= a.1 && b.0 <= a.0 {
        Some(a)
    } else {
        Some((std::cmp::max(a.0, b.0), std::cmp::min(a.1, b.1)))
    }
}

impl Cuboid {
    fn size(&self) -> i64 {
        let x_size = (self.x.1 - self.x.0 + 1) as i64;
        let y_size = (self.y.1 - self.y.0 + 1) as i64;
        let z_size = (self.z.1 - self.z.0 + 1) as i64;
        x_size*y_size*z_size
    }

    // Intersec two cuboids
    fn intersect(&self, other: &Cuboid) -> Option<Cuboid> {
        Some(Cuboid {
            x: intersect_one_axis(self.x, other.x)?,
            y: intersect_one_axis(self.y, other.y)?,
            z: intersect_one_axis(self.z, other.z)?,
        })
    }
}

#[derive(Default)]
struct Reactor {
    // A reactor consists of a set of reboot steps, which each:
    // - lit a cuboid
    // - unlit a cuboid
    // Contrary to the input, those steps are guaranteed to be complete,
    // ie each step will either lit or unlit of the cubes it contains.
    // Computing the number of lit cubes is thus then trivial, we
    // just add/substract all the cubes contained by those steps.
    steps: Vec<RebootStep>,
}

impl Reactor {
    fn apply_reboot_step(&mut self, new_step: RebootStep) {
        // Compute the intersection of the new step with the existing one.
        // Each intersection will generate a new step, that will match how
        // the cubes in this cuboid are modified.
        let steps_to_add: Vec<_> = self.steps.iter().filter_map(|step| {
            step.cuboid.intersect(&new_step.cuboid).and_then(|intersection| {
                // - cuboid on, step off => we add a cuboid for the intersection
                // as an "off" operation.
                //
                // - cuboid on, step on => the step will be added as its own cuboid,
                // so add the intersection as an "off", ie:
                //   - first cuboid is lit
                //   - then intersection is unlit
                //   - then new cuboid is lit
                //
                // - cuboid off, step on => add a "on" cuboid for the intersection
                //
                // - cuboid off, step off => we need to add the intersection as "on"!
                //   This is needed because the "off" cuboid only exists as an "off"
                //   operation on an existing "on" cuboid. This means that this step
                //   will also intersect with this "on" cuboid, which will add an
                //   "off" cuboid on its own. To compensate this since we now have
                //   two "off" cuboids on this intersection, we need to add an "on".
                Some(RebootStep {
                    on: !step.on,
                    cuboid: intersection,
                })
            })
        }).collect();
        self.steps.extend(steps_to_add.into_iter());

        // If the added cuboid is "on", add it to the list.
        if new_step.on {
            self.steps.push(new_step);
        }
    }

    fn count_activated_cuboids(&self) -> i64 {
        self.steps.iter().map(|step| {
            let count = step.cuboid.size();
            if step.on {
                count
            } else {
                -count
            }
        }).sum()
    }
}

#[derive(Debug)]
struct RebootStep {
    on: bool,
    cuboid: Cuboid,
}

fn parse_input(input: &str) -> Vec<RebootStep> {
    input.lines().map(|line| {
        let (on, line) = match line.strip_prefix("on ") {
            Some(v) => (true, v),
            None => (false, line.strip_prefix("off ").unwrap())
        };
        let mut items = line.split(',');
        let x_range = items.next().unwrap().strip_prefix("x=").unwrap();
        let y_range = items.next().unwrap().strip_prefix("y=").unwrap();
        let z_range = items.next().unwrap().strip_prefix("z=").unwrap();
        assert!(items.next().is_none());

        let x = parse_range(x_range);
        let y = parse_range(y_range);
        let z = parse_range(z_range);

        RebootStep {
            on,
            cuboid: Cuboid {
                x, y, z,
            }
        }
    }).collect()
}

fn parse_range(input: &str) -> (i32, i32) {
    let mut items = input.split("..");
    let from = items.next().unwrap().parse().unwrap();
    let to = items.next().unwrap().parse().unwrap();
    assert!(items.next().is_none());
    (from, to)
}
