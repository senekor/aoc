use aoc_15_18::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 4);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 7);
}

utils::solution!(
    aoc_15_18;
    1061;
    1006;
);
