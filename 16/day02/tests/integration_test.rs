#[test]
fn test_part1_sample() {
    let input = include_str!("../input/sample.txt");
    assert_eq!(day02::part1(input), 1985);
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(day02::part1(input), 92435);
}

#[test]
fn test_part2_sample() {
    let input = include_str!("../input/sample.txt");
    assert_eq!(day02::part2(input), 0x5DB3);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(day02::part2(input), 0xC1A88);
}
