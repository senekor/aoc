use aoc_16_05::{part1, part2};

#[test]
#[ignore]
fn test_part1_sample() {
    let input = include_str!("../input/sample.txt");
    assert_eq!(part1(input), "18f47a30");
}

#[test]
#[ignore]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), "801b56a7");
}

#[test]
#[ignore]
fn test_part2_sample() {
    let input = include_str!("../input/sample.txt");
    assert_eq!(part2(input), "05ace8e3");
}

#[test]
#[ignore]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), "424a0197");
}
