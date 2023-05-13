use aoc_18_03::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 4);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 3);
}

utils::solution!(
    aoc_18_03;
    111630;
    724;
);
