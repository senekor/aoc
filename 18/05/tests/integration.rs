use aoc_18_05::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 10);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 10762);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 4);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 6946);
}
