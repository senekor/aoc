use aoc_17_09::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1("{{{}}}"), 6);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 10800);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(r#"{<{o"i!a,<{i<a>}"#), 10);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 4522);
}
