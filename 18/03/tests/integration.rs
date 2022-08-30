use aoc_18_03::{part1, part2};

#[test]
fn test_part1_sample() {
    let input = include_str!("../input/sample_input.txt");
    assert_eq!(part1(input), 4);
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 111630);
}

#[test]
fn test_part2_sample() {
    let input = include_str!("../input/sample_input.txt");
    assert_eq!(part2(input), 3);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), 724);
}
