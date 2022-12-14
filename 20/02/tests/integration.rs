use aoc_20_02::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 2);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 493);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 1);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 593);
}
