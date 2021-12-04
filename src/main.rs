mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    aoc_driver::aoc_complete! {
        session_file: "./.session.txt",
        input_dir: "input"
        challenges: [
            { "2021-1-1": day1::part1 } { "2021-1-2": day1::part2 }
            { "2021-2-1": day2::part1 } { "2021-2-2": day2::part2 }
            {
                "2021-3-1": day3::part1
                tests: [
                    { name: "1", input: "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010", output: "198" }
                ]
            }
            {
                "2021-3-2": day3::part2
                tests: [
                    { name: "1", input: "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010", output: "230" }
                ]
            }
            {
                "2021-4-1": day4::part1
                tests: [
                    { name: "1", input: "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

                    22 13 17 11  0
                     8  2 23  4 24
                    21  9 14 16  7
                     6 10  3 18  5
                     1 12 20 15 19

                     3 15  0  2 22
                     9 18 13 17  5
                    19  8  7 25 23
                    20 11 10 24  4
                    14 21 16 12  6

                    14 21 17 24  4
                    10 16 15  9 19
                    18  8 23 26 20
                    22 11 13  6  5
                     2  0 12  3  7", output: "4512" }
                ]
            }
            {
                "2021-4-2": day4::part2
                tests: [
                    { name: "2", input: "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

                    22 13 17 11  0
                     8  2 23  4 24
                    21  9 14 16  7
                     6 10  3 18  5
                     1 12 20 15 19

                     3 15  0  2 22
                     9 18 13 17  5
                    19  8  7 25 23
                    20 11 10 24  4
                    14 21 16 12  6

                    14 21 17 24  4
                    10 16 15  9 19
                    18  8 23 26 20
                    22 11 13  6  5
                     2  0 12  3  7", output: "1924" }
                ]
            }
        ]
    }
}