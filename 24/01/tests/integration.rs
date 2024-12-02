use aoc_24_01::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 11);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 31);
}

utils::solution!(
    aoc_24_01;
    1938424;
    22014209;
);
