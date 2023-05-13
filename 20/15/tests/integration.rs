use aoc_20_15::part1;

#[allow(unused)]
use utils::fail::Fail;

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

utils::solution!(
    aoc_20_15;
    595;
    1708310;
);
