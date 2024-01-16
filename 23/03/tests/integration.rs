use aoc_23_03::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 4361);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 467835);
}

utils::solution!(
    aoc_23_03;
    532428;
    84051670;
);
