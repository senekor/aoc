use aoc_19_06::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE_1: &str = include_str!("../input/sample_1.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE_1), 42);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE_2), 4);
}

utils::solution!(
    aoc_19_06;
    147807;
    229;
);
