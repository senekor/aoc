use aoc_16_02::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 1985);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 0x5DB3);
}

utils::solution!(
    aoc_16_02;
    92435;
    0xC1A88;
);
