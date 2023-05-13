use aoc_22_11::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 10605);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 2713310158);
}

utils::solution!(
    aoc_22_11;
    50830;
    14399640002;
);
