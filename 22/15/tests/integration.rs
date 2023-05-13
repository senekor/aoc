use aoc_22_15::{non_beacons_at_y, tuning_freq_in};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(non_beacons_at_y::<10>(SAMPLE), 26);
}

#[test]
fn test_part2_sample() {
    assert_eq!(tuning_freq_in::<20>(SAMPLE), 56000011);
}

utils::solution!(
    aoc_22_15;
    5564017;
    11558423398893;
);
