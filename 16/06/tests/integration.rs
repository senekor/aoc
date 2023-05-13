use aoc_16_06::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), "easter");
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), "advent");
}

utils::solution!(
    aoc_16_06;
    "tsreykjj";
    "hnfbujie";
);
