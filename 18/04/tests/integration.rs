use aoc_18_04::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 240);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 4455);
}

utils::solution!(
    aoc_18_04;
    20859;
    76576;
);
