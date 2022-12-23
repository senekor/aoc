use aoc_22_05::{part1, part2};

#[test]
fn test_part1_sample() {
    let input = include_str!("../input/sample.txt");
    assert_eq!(part1(input), "CMZ");
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), "PSNRGBTFT");
}

#[test]
fn test_part2_sample() {
    let input = include_str!("../input/sample.txt");
    assert_eq!(part2(input), "MCD");
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), "BNTZFPMMW");
}
