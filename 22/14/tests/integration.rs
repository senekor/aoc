use aoc_22_14::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 24);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 1061);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 93);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 25055);
}
