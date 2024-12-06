use aoc_24_02::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 2);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 4);
}

utils::solution!(
    aoc_24_02;
    549;
    589;
);
