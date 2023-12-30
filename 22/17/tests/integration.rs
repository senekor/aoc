use aoc_22_17::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 3068);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), Fail);
}

utils::solution!(
    aoc_22_17;
    3106;
    TODO;
);
