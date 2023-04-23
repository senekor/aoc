use utils::Itertools;

struct ParsedInput(Vec<i32>);

fn parse_input(input: &str) -> ParsedInput {
    ParsedInput(
        input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect_vec(),
    )
}

pub fn part1(input: &str) -> i32 {
    let count = parse_input(input)
        .0
        .windows(2)
        .fold(0, |acc, w| if w[0] < w[1] { acc + 1 } else { acc });

    count
}

pub fn part2(input: &str) -> i32 {
    let count = parse_input(input)
        .0
        .windows(4)
        .fold(0, |acc, w| if w[0] < w[3] { acc + 1 } else { acc });

    count
}
