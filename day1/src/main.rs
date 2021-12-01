use itertools::*;

type ParsedInput = Vec<i32>;

fn part1(measurements: &ParsedInput) {
    let count = measurements
        .windows(2)
        .fold(0, |acc, w| if w[0] < w[1] { acc + 1 } else { acc });

    println!("{}", count)
}

fn part2(measurements: &ParsedInput) {
    let measurement_windows = measurements.windows(3).collect_vec();

    let count = measurement_windows.windows(2).fold(0, |acc, mws| {
        let prev_sum = mws[0].iter().sum::<i32>();
        let sum = mws[1].iter().sum();
        if prev_sum < sum {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", count)
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
