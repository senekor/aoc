use aoc_16_07::{part1, part2};

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 118);
}

#[test]
fn test_part2_remo() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), 260);
}
