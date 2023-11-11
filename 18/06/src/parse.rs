use utils::{
    nom::{
        bytes::complete::tag, character::complete::newline, combinator::map,
        multi::separated_list0, sequence::separated_pair, IResult,
    },
    parse::integer::usize,
};

use crate::Coordinate;

fn coordinate(input: &str) -> IResult<&str, Coordinate> {
    map(separated_pair(usize, tag(", "), usize), |(x, y)| {
        Coordinate { x, y }
    })(input)
}

pub(crate) fn coordinates(input: &str) -> IResult<&str, Vec<Coordinate>> {
    separated_list0(newline, coordinate)(input)
}
