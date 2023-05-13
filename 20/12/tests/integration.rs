use aoc_20_12::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 25);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 286);
}

utils::solution!(
    aoc_20_12;
    2297;
    89984;
);
