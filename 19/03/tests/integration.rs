use aoc_19_03::{part1, part2};

static SAMPLE_1: &str = include_str!("../input/sample_1.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");
static SAMPLE_3: &str = include_str!("../input/sample_3.txt");

#[test]
fn test_part1_sample_1() {
    assert_eq!(part1(SAMPLE_1), 6);
}

#[test]
fn test_part1_sample_2() {
    assert_eq!(part1(SAMPLE_2), 159);
}

#[test]
fn test_part1_sample_3() {
    assert_eq!(part1(SAMPLE_3), 135);
}

#[test]
fn test_part2_sample_1() {
    assert_eq!(part2(SAMPLE_1), 30);
}

#[test]
fn test_part2_sample_2() {
    assert_eq!(part2(SAMPLE_2), 610);
}

#[test]
fn test_part2_sample_3() {
    assert_eq!(part2(SAMPLE_3), 410);
}

utils::solution!(
    aoc_19_03;
    709;
    13836;
);
