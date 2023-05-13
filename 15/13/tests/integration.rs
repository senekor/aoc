use aoc_15_13::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 330);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 286);
}

utils::solution!(
    aoc_15_13;
    709;
    668;
);
