use aoc_17_08::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 1);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 4416);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 10);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 5199);
}
