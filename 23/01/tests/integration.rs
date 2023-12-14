use aoc_23_01::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 142);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE_2), 281);
}

utils::solution!(
    aoc_23_01;
    55386;
    54824;
);
