mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
            {
                "2021-5-1": day5::part1
                tests: [
                    { name: "1", input: "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2", output: "5" }
                ]
            }
            {
                "2021-5-2": day5::part2
                tests: [
                    { name: "1", input: "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2", output: "12" }
                ]
            }
            {
                "2021-6-1": day6::part1
                tests: [
                    { name: "1", input: "3,4,3,1,2", output: "5934" }
                ]
            }
            {
                "2021-6-2": day6::part2
                tests: [
                    { name: "1", input: "3,4,3,1,2", output: "26984457539" }
                ]
            }
            {
                "2021-7-1": day7::part1
                tests: [
                    { name: "1", input: "16,1,2,0,4,2,7,1,2,14", output: "37" }
                ]
            }
            {
                "2021-7-1": day7::part1
                tests: [
                    { name: "1", input: "16,1,2,0,4,2,7,1,2,14", output: "37" }
                ]
            }
            {
                "2021-7-2": day7::part2
                tests: [
                    { name: "1", input: "16,1,2,0,4,2,7,1,2,14", output: "168" }
                ]
            }
            {
                "2021-8-1": day8::part1
                tests: [
                    { name: "1", input: "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce", output: "26" }
                ]
            }
            {
                "2021-8-2": day8::part2
                tests: [
                    { name: "1", input: "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce", output: "61229" }
                ]
            }
            {
                "2021-9-1": day9::part1
                tests: [
                    { name: "1", input: "2199943210
3987894921
9856789892
8767896789
9899965678", output: "15" }
                ]
            }
            {
                "2021-9-2": day9::part2
                tests: [
                    { name: "1", input: "2199943210
3987894921
9856789892
8767896789
9899965678", output: "1134" }
                ]
            }
            {
                "2021-10-1": day10::part1
                tests: [
                    { name: "1", input: "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]", output: "26397" }
                ]
            }
            {
                "2021-10-2": day10::part2
                tests: [
                    { name: "1", input: "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]", output: "288957" }
                ]
            }
            {
                "2021-11-1": day11::part1
                tests: [
                    { name: "1", input: "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526", output: "1656" }
                ]
            }
            {
                "2021-11-2": day11::part2
                tests: [
                    { name: "1", input: "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526", output: "195" }
                ]
            }
            {
                "2021-12-1": day12::part1
                tests: [
                    { name: "1", input: "start-A
start-b
A-c
A-b
b-d
A-end
b-end", output: "10" }
                    { name: "1", input: "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc", output: "19" }
                    { name: "1", input: "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW", output: "226" }
                ]
            }
            {
                "2021-12-2": day12::part2
                tests: [
                    { name: "1", input: "start-A
start-b
A-c
A-b
b-d
A-end
b-end", output: "36" }
                    { name: "1", input: "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc", output: "103" }
                    { name: "1", input: "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW", output: "3509" }
                ]
            }
            {
                "2021-13-1": day13::part1
                tests: [
                    { name: "1", input: "6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5", output: "17" }
                ]
            }
        ]
    }
}
