use aoc_17_03::{part1, part2};

#[test]
fn test_part1_sample() {
    assert_eq!(part1("1"), 0);
    assert_eq!(part1("12"), 3);
    assert_eq!(part1("23"), 2);
    assert_eq!(part1("1024"), 31);
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 475);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), 279138);
}
