use aoc_17_06::{part1_impl, part2_impl};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1_impl::<4>(SAMPLE), 5);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2_impl::<4>(SAMPLE), 4);
}

utils::solution!(
    aoc_17_06;
    6681;
    2392;
);
