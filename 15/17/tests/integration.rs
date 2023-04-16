use aoc_15_17::{part1, part2};

static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 1638);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 17);
}