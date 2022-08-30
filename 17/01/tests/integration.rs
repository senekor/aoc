#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(aoc_17_01::part1(input), 1223);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(aoc_17_01::part2(input), 1284);
}
