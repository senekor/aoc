use aoc_17_05::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 5);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 10);
}

utils::solution!(
    aoc_17_05;
    343364;
    25071947;
);
