use aoc_18_06::{part1, part2_impl};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 17);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2_impl(SAMPLE, 32), 16);
}

utils::solution!(
    aoc_18_06;
    3907;
    42036;
);
