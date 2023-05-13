use aoc_17_04::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 2);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 2);
}

utils::solution!(
    aoc_17_04;
    466;
    251;
);
