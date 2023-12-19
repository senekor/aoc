use aoc_22_16::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 1651);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 1707);
}

utils::solution!(
    aoc_22_16;
    ignore 1460;
    ignore 2117;
);
