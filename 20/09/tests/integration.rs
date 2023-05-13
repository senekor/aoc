use aoc_20_09::{part1_impl, part2_impl};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1_impl(SAMPLE, 5), 127);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2_impl(SAMPLE, 5), 62);
}

utils::solution!(
    aoc_20_09;
    167829540;
    28045630;
);
