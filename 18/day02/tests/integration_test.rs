use day02::{part1, part2};

#[test]
fn test_part1_sample() {
    let input = include_str!("../input/sample_input.txt");
    assert_eq!(part1(input), 12);
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 7192);
}

#[test]
fn test_part2_sample() {
    let input = include_str!("../input/sample_input_2.txt");
    assert_eq!(part2(input), "fgij");
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), "mbruvapghxlzycbhmfqjonsie");
}
