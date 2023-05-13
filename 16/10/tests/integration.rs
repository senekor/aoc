use aoc_16_10::part1_impl;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1_impl(SAMPLE, 5, 2), 2);
}

utils::solution!(
    aoc_16_10;
    98;
    4042;
);
