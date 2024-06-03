use aoc_18_10::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), "hi");
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 3);
}

utils::solution!(
    aoc_18_10;
    "xecxbpzb";
    10124;
);
