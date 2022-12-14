use aoc_20_09::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE, 5), 127);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT, 25), 167829540);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE, 5), 62);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT, 25), 28045630);
}
