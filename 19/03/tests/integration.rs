use aoc_19_03::{part1, part2};

#[test]
fn test_part1_sample_1() {
    let input = include_str!("../input/sample_1.txt");
    assert_eq!(part1(input), 6);
}

#[test]
fn test_part1_sample_2() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part1(input), 159);
}

#[test]
fn test_part1_sample_3() {
    let input = include_str!("../input/sample_3.txt");
    assert_eq!(part1(input), 135);
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 709);
}

#[test]
fn test_part2_sample_1() {
    let input = include_str!("../input/sample_1.txt");
    assert_eq!(part2(input), 30);
}

#[test]
fn test_part2_sample_2() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part2(input), 610);
}

#[test]
fn test_part2_sample_3() {
    let input = include_str!("../input/sample_3.txt");
    assert_eq!(part2(input), 410);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), 13836);
}
