use utils::winnow::{
    ascii::{digit1, newline},
    combinator::{separated, separated_pair},
    PResult, Parser,
};

use crate::{Path, Point};

fn usize(input: &mut &str) -> PResult<usize> {
    digit1
        .map(|digs: &str| digs.parse().unwrap())
        .parse_next(input)
}

fn point(input: &mut &str) -> PResult<Point> {
    separated_pair(usize, ',', usize)
        .map(|(x, y)| Point::new(x, y))
        .parse_next(input)
}

fn path(input: &mut &str) -> PResult<Path> {
    separated(1.., point, " -> ").parse_next(input)
}

pub(crate) fn paths(input: &mut &str) -> PResult<Vec<Path>> {
    separated(.., path, newline).parse_next(input)
}
