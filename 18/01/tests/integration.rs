use aoc_18_01::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 3);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 525);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2("+1\n-2\n+3\n+1"), 2);
    assert_eq!(part2("+1\n-1"), 0);
    assert_eq!(part2("+3\n+3\n+4\n-2\n-4"), 10);
    assert_eq!(part2("-6\n+3\n+8\n+5\n-6"), 5);
    assert_eq!(part2("+7\n+7\n-2\n-7\n-4"), 14);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 75749);
}
