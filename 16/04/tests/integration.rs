use aoc_16_04::part1;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 1514);
}

utils::solution!(
    aoc_16_04;
    361724;
    482;
);
