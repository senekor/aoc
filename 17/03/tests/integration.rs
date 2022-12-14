use aoc_17_03::{part1, part2};

static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1("1"), 0);
    assert_eq!(part1("12"), 3);
    assert_eq!(part1("23"), 2);
    assert_eq!(part1("1024"), 31);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 475);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 279138);
}
