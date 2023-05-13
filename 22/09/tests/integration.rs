use aoc_22_09::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 13);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 1);
}

#[test]
fn test_part2_sample_2() {
    assert_eq!(part2(SAMPLE_2), 36);
}

utils::solution!(
    aoc_22_09;
    6486;
    2678;
);
