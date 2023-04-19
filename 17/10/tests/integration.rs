use aoc_17_10::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 20056);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), "d9a7de4a809c56bf3a9465cb84392c8e");
}
