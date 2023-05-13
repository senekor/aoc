use aoc_20_17::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 112);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 848);
}

utils::solution!(
    aoc_20_17;
    289;
    2084;
);
