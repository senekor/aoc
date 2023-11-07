use aoc_18_07::{part1, part2_impl};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), "CABDFE");
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2_impl(SAMPLE, 2, 0), 15);
}

utils::solution!(
    aoc_18_07;
    "EBICGKQOVMYZJAWRDPXFSUTNLH";
    906;
);
