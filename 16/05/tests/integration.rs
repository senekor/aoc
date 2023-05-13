use aoc_16_05::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
#[ignore]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), "18f47a30");
}

#[test]
#[ignore]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE), "05ace8e3");
}

utils::solution!(
    aoc_16_05;
    ignore "801b56a7";
    ignore "424a0197";
);
