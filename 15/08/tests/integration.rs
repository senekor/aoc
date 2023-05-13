use aoc_15_08::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 12);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 19);
}

utils::solution!(
    aoc_15_08;
    1342;
    2074;
);
