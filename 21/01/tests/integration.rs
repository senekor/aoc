use aoc_21_01::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 7);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 5);
}

utils::solution!(
    aoc_21_01;
    1527;
    1575;
);
