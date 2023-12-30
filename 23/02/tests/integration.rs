use aoc_23_02::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 8);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 2286);
}

utils::solution!(
    aoc_23_02;
    2006;
    84911;
);
