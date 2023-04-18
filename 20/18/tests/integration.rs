use aoc_20_18::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");
static SAMPLE_3: &str = include_str!("../input/sample_3.txt");
static SAMPLE_4: &str = include_str!("../input/sample_4.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 26);
}

#[test]
fn test_part1_sample_2() {
    assert_eq!(part1(SAMPLE_2), 437);
}

#[test]
fn test_part1_sample_3() {
    assert_eq!(part1(SAMPLE_3), 12240);
}

#[test]
fn test_part1_sample_4() {
    assert_eq!(part1(SAMPLE_4), 13632);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 6923486965641);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), 46);
}

#[test]
fn test_part2_sample_2() {
    assert_eq!(part2(SAMPLE_2), 1445);
}

#[test]
fn test_part2_sample_3() {
    assert_eq!(part2(SAMPLE_3), 669060);
}

#[test]
fn test_part2_sample_4() {
    assert_eq!(part2(SAMPLE_4), 23340);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 70722650566361);
}
