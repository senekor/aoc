use aoc_20_07::{part1, part2};

static SAMPLE_1: &str = include_str!("../input/sample_1.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE_1), 4);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 179);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE_2), 126);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 18925);
}
