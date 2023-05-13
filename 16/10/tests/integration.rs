use aoc_16_10::{part1, part1_impl, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1_impl(SAMPLE, 5, 2), 2);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 98);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 4042);
}
