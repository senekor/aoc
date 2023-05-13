use aoc_15_14::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 2660);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 1564);
}

utils::solution!(
    aoc_15_14;
    2696;
    1084;
);
