use aoc_20_14::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 165);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 11179633149677);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE_2), 208);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 4822600194774);
}
