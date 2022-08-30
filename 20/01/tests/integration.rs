use aoc_20_01::{part1, part2};

#[test]
fn test_part1_sample() {
    let input = include_str!("../input/sample_input.txt");
    assert_eq!(part1(input), 514579)
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 840324)
}

#[test]
fn test_part2_sample() {
    let input = include_str!("../input/sample_input.txt");
    assert_eq!(part2(input), 241861950)
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), 170098110)
}
