use aoc_20_13::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 295);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 1068781);
}

utils::solution!(
    aoc_20_13;
    370;
    894954360381385;
);
