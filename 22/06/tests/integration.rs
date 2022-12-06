use aoc_22_06::{part1, part2};

#[test]
fn test_part1_sample() {
    let input = include_str!("../input/sample.txt");
    assert_eq!(part1(input), 7);
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 1723);
}

#[test]
fn test_part2_sample() {
    let input = include_str!("../input/sample.txt");
    assert_eq!(part2(input), 19);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), 3708);
}
