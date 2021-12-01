mod day1;

fn main() {
    aoc_driver::aoc_complete! {
        session_file: "./.session.txt",
        input_dir: "input"
        challenges: [
            {
                "2021-1-1": day1::part1,
            }
            {
                "2021-1-2": day1::part2,
            }
        ]
    }
}
