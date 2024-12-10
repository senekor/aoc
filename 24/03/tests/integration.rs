use aoc_24_03::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");
static SAMPLE_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 161);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE_2), 48);
}

utils::solution!(
    aoc_24_03;
    189600467;
    107069718;
);
