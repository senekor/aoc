use itertools::izip;

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
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .chunks(9)
        .flat_map(|nine| {
            izip!(
                nine[0..3].iter().copied(),
                nine[3..6].iter().copied(),
                nine[6..9].iter().copied(),
            )
        })
        .map(Triangle::from_tuple)
        .filter(Triangle::is_possible)
        .count()
}
