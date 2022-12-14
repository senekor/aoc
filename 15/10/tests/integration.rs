use aoc_15_10::{part1, part2};

use num::BigUint;
static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), BigUint::from(237746_u32));
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), BigUint::from(360154_u32));
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), BigUint::from(3369156_u32));
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), BigUint::from(5103798_u32));
}
