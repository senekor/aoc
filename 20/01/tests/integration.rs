use aoc_20_01::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 514579);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 241861950);
}

utils::solution!(
    aoc_20_01;
    840324;
    170098110;
);
