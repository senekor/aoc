use aoc_17_07::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), "tknk");
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 60);
}

utils::solution!(
    aoc_17_07;
    "eqgvf";
    757;
);
