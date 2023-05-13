use aoc_22_02::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 15);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 12);
}

utils::solution!(
    aoc_22_02;
    11767;
    13886;
);
