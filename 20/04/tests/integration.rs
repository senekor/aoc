use aoc_20_04::part1;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 2);
}

utils::solution!(
    aoc_20_04;
    242;
    186;
);
