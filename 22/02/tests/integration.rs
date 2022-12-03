use aoc_22_02::{part1, part2};

#[test]
fn test_part1_sample() {
    let input = include_str!("../input/sample.txt");
    assert_eq!(part1(input), 15);
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 11767);
}

#[test]
fn test_part2_sample() {
    let input = include_str!("../input/sample.txt");
    assert_eq!(part2(input), 12);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), 13886);
}
