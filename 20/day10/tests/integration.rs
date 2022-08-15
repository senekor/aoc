use day10::{part1, part2};

#[test]
fn test_part1_sample_1() {
    let input = include_str!("../input/sample_1.txt");
    assert_eq!(part1(input), 35);
}

#[test]
fn test_part1_sample_2() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part1(input), 220);
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 2112);
}

#[test]
fn test_part2_sample_1() {
    let input = include_str!("../input/sample_1.txt");
    assert_eq!(part2(input), 8);
}

#[test]
fn test_part2_sample_2() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part2(input), 19208);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), 3022415986688);
}
