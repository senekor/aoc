use aoc_17_06::{part1, part1_impl, part2, part2_impl};

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1_impl::<4>(SAMPLE), 5);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 6681);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2_impl::<4>(SAMPLE), 4);
}

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), 2392);
}
