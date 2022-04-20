use day05::{part1, part2};

#[test]
fn test_part1_sample() {
    let input = include_str!("../input/sample.txt");
    assert_eq!(part1(input), 10);
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 10762);
}

#[test]
fn test_part2_sample() {
    let input = include_str!("../input/sample.txt");
    assert_eq!(part2(input), 4);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), 6946);
}
