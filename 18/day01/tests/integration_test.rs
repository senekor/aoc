use day01::{part1, part2};

#[test]
fn test_part1_sample() {
    let input = include_str!("../input/sample_input.txt");
    assert_eq!(part1(input), 3)
}

#[test]
fn test_part1() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part1(input), 525)
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2("+1\n-2\n+3\n+1"), 2);
    assert_eq!(part2("+1\n-1"), 0);
    assert_eq!(part2("+3\n+3\n+4\n-2\n-4"), 10);
    assert_eq!(part2("-6\n+3\n+8\n+5\n-6"), 5);
    assert_eq!(part2("+7\n+7\n-2\n-7\n-4"), 14);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(part2(input), 75749)
}
