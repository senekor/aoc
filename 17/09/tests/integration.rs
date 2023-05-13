use aoc_17_09::{part1, part2};

#[allow(unused)]
use utils::fail::Fail;

#[test]
fn test_part1_sample() {
    assert_eq!(part1("{{{}}}"), 6);
}

#[test]
fn test_part2_sample() {
    assert_eq!(part2(r#"{<{o"i!a,<{i<a>}"#), 10);
}

utils::solution!(
    aoc_17_09;
    10800;
    4522;
);
