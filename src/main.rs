mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
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
            {
                "2021-14-1": day14::part1
                tests: [
                    { name: "1", input: "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C\n", output: "1588" }
                ]
            }
            {
                "2021-14-2": day14::part2
                tests: [
                    { name: "1", input: "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C\n", output: "2188189693529" }
                ]
            }
            {
                "2021-15-1": day15::part1
                tests: [
                    { name: "1", input: "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581", output: "40" }
                ]
            }
            {
                "2021-15-2": day15::part2
                tests: [
                    { name: "1", input: "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581", output: "315" }
                ]
            }
            {
                "2021-16-1": day16::part1
                tests: [
                    { name: "0", input: "D2FE28", output: "6" }
                    { name: "1", input: "38006F45291200", output: "9" }
                    { name: "2", input: "EE00D40C823060", output: "14" }
                    { name: "3", input: "8A004A801A8002F478", output: "16" }
                    { name: "4", input: "620080001611562C8802118E34", output: "12" }
                    { name: "5", input: "C0015000016115A2E0802F182340", output: "23" }
                    { name: "6", input: "A0016C880162017C3686B18A3D4780", output: "31" }
                ]
            }
            {
                "2021-16-2": day16::part2
                tests: [
                    { name: "0", input: "C200B40A82", output: "3" }
                    { name: "1", input: "04005AC33890", output: "54" }
                    { name: "2", input: "880086C3E88112", output: "7" }
                    { name: "3", input: "CE00C43D881120", output: "9" }
                    { name: "4", input: "D8005AC2A8F0", output: "1" }
                    { name: "5", input: "F600BC2D8F", output: "0" }
                    { name: "6", input: "9C005AC2F8F0", output: "0" }
                    { name: "7", input: "9C0141080250320F1802104A08", output: "1" }
                ]
            }
            {
                "2021-17-1": day17::part1
                tests: [
                    { name: "0", input: "target area: x=20..30, y=-10..-5", output: "45" }
                ]
            }
            {
                "2021-17-2": day17::part2
                tests: [
                    { name: "0", input: "target area: x=20..30, y=-10..-5", output: "112" }
                ]
            }
            {
                "2021-18-1": day18::part1
                tests: [
                    { name: "0", input: "[9,1]", output: "29" }
                    { name: "1", input: "[[9,1],[1,9]]", output: "129" }
                    { name: "2", input: "[[1,2],[[3,4],5]]", output: "143" }
                    { name: "3", input: "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", output: "1384" }
                    { name: "4", input: "[[[[1,1],[2,2]],[3,3]],[4,4]]", output: "445" }
                    { name: "5", input: "[[[[3,0],[5,3]],[4,4]],[5,5]]", output: "791" }
                    { name: "6", input: "[[[[5,0],[7,4]],[5,5]],[6,6]]", output: "1137" }
                    { name: "7", input: "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]", output: "4140" }
                ]
            }
            {
                "2021-18-2": day18::part2
                tests: [
                    { name: "7", input: "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]", output: "3993" }
                ]
            }
            {
                "2021-19-1": day19::part1
                tests: [
                    { name: "0", input: "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14", output: "79" }
                ]
            }
            {
                "2021-19-1": day19::part2
                tests: [
                    { name: "0", input: "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14", output: "3621" }
                ]
            }
        ]
    }
}
