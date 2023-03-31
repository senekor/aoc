use aoc_22_07::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 95437);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 1453349);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 24933642);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 2948823);
}
