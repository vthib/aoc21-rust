pub fn part1(input: &str) -> impl std::fmt::Display {
    let goal = Goal::from_str(input);

    // We want to find the (vx0, vy0) initial velocity that gives the max y
    // value.
    // For x, we do not really care, a value that leads to having 0 x
    // velocity while x is in the goal range // is enough.
    let vy0 = goal.get_max_initial_y();

    // Now that we have vy0, we can compute the max y reached. This is
    // vy0 + (vy0 - 1) + ...
    compute_arc_distance(vy0)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let goal = Goal::from_str(input);

    // We want to find all initial vectors that will reach the goal.
    // We could try to be smart, but bruteforcing it will be enough:
    // - Find which vx reaches the goal on the x axis
    // - Find vy0min and vy0max
    // - For every vy between those, check if it reaches the goal on the y axis

    // vx0min is first arc that reaches in the goal
    let mut vx0min = 0;
    while compute_arc_distance(vx0min) < goal.x_min {
        vx0min += 1;
    }

    // vx0max is reaching the goal_x_max in one tick.
    let vx0max = goal.x_max;

    // vy0min is first arc that reaches the goal. But since y can
    // be negative, this is not as easy to find as for x
    let vy0min = if goal.y_min < 0 {
        // goal min value is negative. "smallest" arc to reach it
        // is a straight boomer from (0, 0) to (goal_x_min, goal_y_min),
        // so from 0 to goal_y_min
        goal.y_min
    } else {
        let mut v = 0;
        while compute_arc_distance(v) < goal.y_min {
            v += 1;
        }
        v
    };

    let vy0max = goal.get_max_initial_y();

    let mut nb_vectors_successful = 0;
    for vx0 in vx0min..=vx0max {
        for vy0 in vy0min..=vy0max {
            if goal.reached_with_velocity_vector(vx0, vy0) {
                nb_vectors_successful += 1;
            }
        }
    }
    nb_vectors_successful
}

// Compute full distance reached from an arc where velocity decreases by 1
// every tick. ie, compute n + (n - 1) + (n - 2) + ... + 1.
fn compute_arc_distance(n: i32) -> i32 {
    n * (n + 1) / 2
}

struct Goal {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Goal {
    fn reached_with_velocity_vector(&self, mut vx: i32, mut vy: i32) -> bool {
        let mut x = 0;
        let mut y = 0;

        while y > self.y_min {
            x += vx;
            y += vy;
            if vx > 0 {
                vx -= 1;
            }
            vy -= 1;

            if x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max {
                return true;
            }
        }
        false
    }

    fn get_max_initial_y(&self) -> i32 {
        use std::cmp::Ordering;

        match self.y_min.cmp(&0) {
            Ordering::Less => {
                // We can see that for an initial (vx0, vy0) velocity, the probe
                // will reach the position (X, 0) with velocity (VX, -vy0) (ie, the
                // probe will have a position with the same y coordinate, but with inverse
                // velocity). What we then want is to reach y_min on the next tick, that
                // will max out the vy0 value.
                -self.y_min - 1
            }
            Ordering::Greater => {
                // Here we want to have the previous tick of the (X, 0) position with
                // the (X', y_min) position.
                self.y_min
            }
            Ordering::Equal => {
                // Here we can have infinite initial vy... Should probably not happen
                // on given selfs.
                unreachable!();
            }
        }
    }

    fn from_str(input: &str) -> Self {
        let input = input.trim().strip_prefix("target area: x=").unwrap();
        let mut elems = input.split(", y=");

        let x = elems.next().unwrap();
        let mut xelems = x.split("..");
        let x_min = xelems.next().unwrap().parse().unwrap();
        let x_max = xelems.next().unwrap().parse().unwrap();

        let y = elems.next().unwrap();
        let mut yelems = y.split("..");
        let y_min = yelems.next().unwrap().parse().unwrap();
        let y_max = yelems.next().unwrap().parse().unwrap();

        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}
