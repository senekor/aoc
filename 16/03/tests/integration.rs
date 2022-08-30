#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(aoc_16_03::part1(input), 982);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(aoc_16_03::part2(input), 1826);
}
