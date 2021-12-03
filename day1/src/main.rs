use itertools::*;

type ParsedInput = Vec<i32>;

fn part1(measurements: &ParsedInput) {
    let count = measurements
        .windows(2)
        .fold(0, |acc, w| if w[0] < w[1] { acc + 1 } else { acc });

    println!("{}", count) // 1527
}

fn part2(measurements: &ParsedInput) {
    let count = measurements
        .windows(4)
        .fold(0, |acc, w| if w[0] < w[3] { acc + 1 } else { acc });

    println!("{}", count) // 1575
}

fn main() {
    let input = include_str!("../input/input.txt");

    let measurements = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect_vec();

    part1(&measurements);

    part2(&measurements);
}
