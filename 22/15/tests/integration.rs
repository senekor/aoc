use aoc_22_15::{part1, part2, non_beacons_at_y};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(non_beacons_at_y::<10>(SAMPLE), 26);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 5564017);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), Fail);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), Fail);
}
