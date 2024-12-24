use aoc_24_04::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 18);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 9);
}

utils::solution!(
    aoc_24_04;
    2454;
    1858;
);
