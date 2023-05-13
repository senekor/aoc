use aoc_20_06::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 11);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 6);
}

utils::solution!(
    aoc_20_06;
    6273;
    3254;
);
