use aoc_22_10::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 0);
}

#[test]
fn test_part1_sample_2() {
    assert_eq!(part1(SAMPLE_2), 13140);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 16480);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), Fail);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), Fail);
}
