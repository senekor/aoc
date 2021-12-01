use itertools::*;
use std::str::FromStr;

// #[derive(Clone, Debug)]
type ParsedInput = Vec<i32>;

// impl FromStr for ParsedInput {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(ParsedInput)
//     }
// }

fn part1(input: &ParsedInput) {
    println!("{:?}", input)
}

fn part2(input: &ParsedInput) {
    println!("{:?}", input)
}

fn main() {
    let input = include_str!("../input/input.txt");

    let parsed_input: ParsedInput = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();

    part1(&parsed_input);

    part2(&parsed_input);
}
