use day07::{part1, part2};

#[test]
fn test_part1_silvia() {
    let input = include_str!("../input/input_silvia.txt");
    assert_eq!(part1(input), 105);
}

#[test]
fn test_part1_remo() {
    let input = include_str!("../input/input_remo.txt");
    assert_eq!(part1(input), 118);
}

#[test]
fn test_part2_silvia() {
    let input = include_str!("../input/input_silvia.txt");
    assert_eq!(part2(input), 258);
}

#[test]
fn test_part2_remo() {
    let input = include_str!("../input/input_remo.txt");
    assert_eq!(part2(input), 260);
}
