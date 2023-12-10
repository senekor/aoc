use utils::winnow::{
    ascii::{dec_uint, newline},
    combinator::{separated, separated_pair},
    PResult, Parser,
};

use crate::Coordinate;

fn coordinate(input: &mut &str) -> PResult<Coordinate> {
    separated_pair(dec_uint, ", ", dec_uint)
        .map(|(x, y): (u64, u64)| Coordinate {
            x: x as usize,
            y: y as usize,
        })
        .parse_next(input)
}

pub(crate) fn coordinates(input: &mut &str) -> PResult<Vec<Coordinate>> {
    separated(.., coordinate, newline).parse_next(input)
}
