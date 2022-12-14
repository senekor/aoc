use aoc_21_13::{part1, part2};

static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

#[test]
fn test_part1_sample() {
    assert_eq!(part1(SAMPLE), 17);
}

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT), 751);
}

const PART2_RES: &str = "\
###...##..#..#.###..#..#.#....#..#.#....
#..#.#..#.#..#.#..#.#.#..#....#.#..#....
#..#.#....####.#..#.##...#....##...#....
###..#.##.#..#.###..#.#..#....#.#..#....
#....#..#.#..#.#.#..#.#..#....#.#..#....
#.....###.#..#.#..#.#..#.####.#..#.####.
";

#[test]
fn test_part2() {
    assert_eq!(part2(INPUT), PART2_RES);
}
