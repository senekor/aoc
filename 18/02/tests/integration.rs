use aoc_18_02::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 12);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), "abcde");
}

utils::solution!(
    aoc_18_02;
    7192;
    "mbruvapghxlzycbhmfqjonsie";
);
