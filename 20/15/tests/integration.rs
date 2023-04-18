use aoc_20_15::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn part1_samples() {
    let samples = [
        ("0,3,6", 436),
        ("1,3,2", 1),
        ("2,1,3", 10),
        ("1,2,3", 27),
        ("2,3,1", 78),
        ("3,2,1", 438),
        ("3,1,2", 1836),
    ];
    for (input, expected) in samples {
        assert_eq!(part1(input), expected);
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 595);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 1708310);
}
