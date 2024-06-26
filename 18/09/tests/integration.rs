use aoc_18_09::part1;

#[allow(unused)]
use utils::fail::Fail;

#[test]
fn test_part1_sample() {
    let cases = [
        ("9 players; last marble is worth 25 points", 32),
        ("10 players; last marble is worth 1618 points", 8317),
        ("13 players; last marble is worth 7999 points", 146373),
        ("17 players; last marble is worth 1104 points", 2764),
        ("21 players; last marble is worth 6111 points", 54718),
        ("30 players; last marble is worth 5807 points", 37305),
    ];
    for (input, expected) in cases {
        let actual = part1(input);
        assert_eq!(
            actual, expected,
            "{input}: was {actual}, should be {expected}"
        )
    }
}

utils::solution!(
    aoc_18_09;
    382055;
    ignore 3133277384;
);
