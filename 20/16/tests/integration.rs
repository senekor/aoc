use aoc_20_16::part1;

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 71);
}

utils::solution!(
    aoc_20_16;
    22073;
    1346570764607;
);
