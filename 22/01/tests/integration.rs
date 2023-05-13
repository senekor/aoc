use aoc_22_01::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 24000);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 45000);
}

utils::solution!(
    aoc_22_01;
    69883;
    207576;
);
