use aoc_21_19::{part1, part2};

static INPUT: &str = include_str!("../input/input.txt");

#[ignore]
#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 408);
}

#[ignore]
#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 13348);
}
