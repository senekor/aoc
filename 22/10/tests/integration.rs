use aoc_22_10::part1;

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE: &str = include_str!("../input/sample.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 0);
}

#[test]
fn test_part1_sample_2() {
    assert_eq!(part1(SAMPLE_2), 13140);
}

static PART2_SOLUTION: &str = "
###..#....####.####.#..#.#....###..###..
#..#.#....#....#....#..#.#....#..#.#..#.
#..#.#....###..###..#..#.#....#..#.###..
###..#....#....#....#..#.#....###..#..#.
#....#....#....#....#..#.#....#....#..#.
#....####.####.#.....##..####.#....###..";

utils::solution!(
    aoc_22_10;
    16480;
    PART2_SOLUTION;
);
