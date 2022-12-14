use aoc_16_05::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
#[ignore]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), "18f47a30");
}

#[test]
#[ignore]
fn test_part1() {
    assert_eq!(part1(INPUT), "801b56a7");
}

#[test]
#[ignore]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), "05ace8e3");
}

#[test]
#[ignore]
fn test_part2() {
    assert_eq!(part2(INPUT), "424a0197");
}
