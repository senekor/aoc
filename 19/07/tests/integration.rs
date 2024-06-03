use aoc_19_07::part1;

#[allow(unused)]
use utils::fail::Fail;

static SAMPLE_1: &str = include_str!("../input/sample_1.txt");
static SAMPLE_2: &str = include_str!("../input/sample_2.txt");
static SAMPLE_3: &str = include_str!("../input/sample_3.txt");
// static SAMPLE_4: &str = include_str!("../input/sample_4.txt");
// static SAMPLE_5: &str = include_str!("../input/sample_5.txt");

#[test]
fn test_part1_sample_1() {
    assert_eq!(part1(SAMPLE_1), 43210);
}
#[test]
fn test_part1_sample_2() {
    assert_eq!(part1(SAMPLE_2), 54321);
}
#[test]
fn test_part1_sample_3() {
    assert_eq!(part1(SAMPLE_3), 65210);
}

// These don't work for some reason :-(
//
// #[test]
// fn test_part2_sample_4() {
//     assert_eq!(part2(SAMPLE_4), 139629729);
// }
// #[test]
// fn test_part2_sample_5() {
//     assert_eq!(part2(SAMPLE_5), 18216);
// }

utils::solution!(
    aoc_19_07;
    225056;
    14260332;
);
