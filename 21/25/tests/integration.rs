use aoc_21_25::part1;

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 58);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 516);
}
