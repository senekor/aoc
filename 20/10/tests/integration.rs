use aoc_20_10::{part1, part2};

static SAMPLE_1: &str = include_str!("../input/sample_1.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample_1() {
    assert_eq!(part1(SAMPLE_1), 35);
}

#[test]
fn test_part1_sample_2() {
    assert_eq!(part1(SAMPLE_2), 220);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 2112);
}

#[test]
fn test_part2_sample_1() {
    assert_eq!(part2(SAMPLE_1), 8);
}

#[test]
fn test_part2_sample_2() {
    assert_eq!(part2(SAMPLE_2), 19208);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 3022415986688);
}
