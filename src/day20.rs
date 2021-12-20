pub fn part1(input: &str) -> impl std::fmt::Display {
    inner(input, 2)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    inner(input, 50)
}

fn inner(input: &str, nb_turns: usize) -> u64 {
    let (enhancement_algo, mut image) = parse_input(input);

    for _ in 0..nb_turns {
        image = image.enhance(&enhancement_algo);
    }

    image.nb_pixels_lit()
}

struct Image {
    // Lines of the image, true is #, false is .
    lines: Vec<Vec<bool>>,
    // Default value outside of the image.
    default_value: bool,
    width: i64,
    height: i64,
}

impl Image {
    fn nb_pixels_lit(&self) -> u64 {
        self.lines
            .iter()
            .map(|line| line.iter().fold(0, |acc, v| if *v { acc + 1 } else { acc }))
            .sum()
    }

    fn enhance(&self, algo: &[bool]) -> Self {
        // Create a new image, with one additional pixel in every direction.
        let w = self.width + 2;
        let h = self.height + 2;

        let lines = (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| {
                        // (x,y) in the new grid is equivalent to
                        // (x-1,y-1) in the old one
                        algo[self.neighborhood_to_int(x - 1, y - 1)]
                    })
                    .collect()
            })
            .collect();

        Self {
            lines,
            default_value: algo[if self.default_value { 511 } else { 0 }],
            width: w,
            height: w,
        }
    }

    fn neighborhood_to_int(&self, x: i64, y: i64) -> usize {
        let mut res = 0;

        res = (res << 1) | self.get_pixel_value(x - 1, y - 1);
        res = (res << 1) | self.get_pixel_value(x, y - 1);
        res = (res << 1) | self.get_pixel_value(x + 1, y - 1);
        res = (res << 1) | self.get_pixel_value(x - 1, y);
        res = (res << 1) | self.get_pixel_value(x, y);
        res = (res << 1) | self.get_pixel_value(x + 1, y);
        res = (res << 1) | self.get_pixel_value(x - 1, y + 1);
        res = (res << 1) | self.get_pixel_value(x, y + 1);
        res = (res << 1) | self.get_pixel_value(x + 1, y + 1);

        res
    }

    fn get_pixel_value(&self, x: i64, y: i64) -> usize {
        let v = if x < 0 || y < 0 || x >= (self.width as i64) || y >= (self.height as i64) {
            self.default_value
        } else {
            self.lines[y as usize][x as usize]
        };
        if v {
            1
        } else {
            0
        }
    }
}

fn parse_input(input: &str) -> (Vec<bool>, Image) {
    let mut parts = input.split("\n\n");
    let algo = parts.next().unwrap();
    let image_lines = parts.next().unwrap();

    let algo: Vec<_> = algo.chars().map(|c| c == '#').collect();
    assert_eq!(algo.len(), 512);
    let lines: Vec<Vec<_>> = image_lines
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let width = lines[0].len() as i64;
    let height = lines.len() as i64;

    (
        algo,
        Image {
            lines,
            default_value: false,
            width,
            height,
        },
    )
}
