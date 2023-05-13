use aoc_20_11::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 37);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 26);
}

utils::solution!(
    aoc_20_11;
    2263;
    2002;
);
