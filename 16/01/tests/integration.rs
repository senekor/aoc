use aoc_16_01::part1;

static SAMPLE: &str = include_str!("../input/sample.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 12);
}

utils::solution!(
    aoc_16_01;
    209;
    136;
);
