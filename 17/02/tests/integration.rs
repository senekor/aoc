use aoc_17_02::{part1, part2};

static SAMPLE_1: &str = include_str!("../input/sample_1.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE_1), 18);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE_2), 9);
}

utils::solution!(
    aoc_17_02;
    36174;
    244;
);
