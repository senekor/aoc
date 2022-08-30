use aoc_17_02::{part1, part2};

#[test]
fn test_part1_sample() {
    let input = include_str!("../input/sample_1.txt");
    assert_eq!(part1(input), 18);
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 36174);
}

#[test]
fn test_part2_sample() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part2(input), 9);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), 244);
}
