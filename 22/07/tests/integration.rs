use aoc_22_07::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 95437);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 24933642);
}

utils::solution!(
    aoc_22_07;
    1453349;
    2948823;
);
