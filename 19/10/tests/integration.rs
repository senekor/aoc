use aoc_19_10::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE_1: &str = include_str!("../input/sample_1.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");
static SAMPLE_3: &str = include_str!("../input/sample_3.txt");
static SAMPLE_4: &str = include_str!("../input/sample_4.txt");
static SAMPLE_5: &str = include_str!("../input/sample_5.txt");

#[test]
fn test_part1_sample_1() {
    assert_eq!(part1(SAMPLE_1), 8);
}
#[test]
fn test_part1_sample_2() {
    assert_eq!(part1(SAMPLE_2), 33);
}
#[test]
fn test_part1_sample_3() {
    assert_eq!(part1(SAMPLE_3), 35);
}
#[test]
fn test_part1_sample_4() {
    assert_eq!(part1(SAMPLE_4), 41);
}
#[test]
fn test_part1_sample_5() {
    assert_eq!(part1(SAMPLE_5), 210);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(SAMPLE_5), 802);
}

utils::solution!(
    aoc_19_10;
    286;
    504;
);
