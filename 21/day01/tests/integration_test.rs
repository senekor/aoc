use day01::lib_main;

#[test]
fn test_part1_and_part2_sample() {
    let input = include_str!("../input/sample_input.txt");
    assert_eq!(lib_main(input), (7, 5))
}

#[test]
fn test_part1_and_part2() {
    let input = include_str!("../input/input.txt");
    assert_eq!(lib_main(input), (1527, 1575))
}
