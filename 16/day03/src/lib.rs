use itertools::{self, Itertools};

mod triangle;
use triangle::Triangle;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| line.parse::<Triangle>().ok())
        .filter(|triangle| triangle.is_possible())
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|v| v.parse::<u32>().unwrap()))
        .chunks(3)
        .into_iter()
        .flat_map(|mut chunk| {
            let mut line_one = chunk.next().unwrap();
            let mut line_two = chunk.next().unwrap();
            let mut line_three = chunk.next().unwrap();
            (0..3)
                .map(|_| {
                    let a = line_one.next().unwrap();
                    let b = line_two.next().unwrap();
                    let c = line_three.next().unwrap();
                    Triangle::new(vec![a, b, c])
                })
                .collect::<Vec<_>>()
        })
        .filter(|triangle| triangle.is_possible())
        .count()
}
