use day03::part1;

#[test]
fn test_part1_sample() {
    assert_eq!(part1(1), 0);
    assert_eq!(part1(12), 3);
    assert_eq!(part1(23), 2);
    assert_eq!(part1(1024), 31);
}

#[test]
fn test_part1() {
    assert_eq!(part1(277678), 475);
}
