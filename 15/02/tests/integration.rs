use aoc_15_02::{part1, part2};

static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 1598415);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 3812909);
}
