use aoc_22_15::{non_beacons_at_y, part1, part2, tuning_freq_in};

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
    assert_eq!(tuning_freq_in::<20>(SAMPLE), 56000011);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 11558423398893);
}
