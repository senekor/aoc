use aoc_16_06::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), "easter");
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), "tsreykjj");
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), "advent");
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), "hnfbujie");
}
