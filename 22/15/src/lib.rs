use nalgebra::Point2;

mod parse;

type Point = Point2<i32>;

pub fn part1(input: &str) -> usize {
    let (_, reports) = parse::reports(input).unwrap();
    0
}

pub fn part2(input: &str) -> usize {
    let (_, _reports) = parse::reports(input).unwrap();
    0
}
