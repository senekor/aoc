use utils::{
    nom::{
        bytes::complete::tag,
        character::complete::{char, newline},
        combinator::map,
        multi::{separated_list0, separated_list1},
        sequence::separated_pair,
        IResult,
    },
    parse::integer::usize,
};

use crate::{Path, Point};

fn point(input: &str) -> IResult<&str, Point> {
    map(separated_pair(usize, char(','), usize), |(x, y)| {
        Point::new(x, y)
    })(input)
}

fn path(input: &str) -> IResult<&str, Path> {
    separated_list1(tag(" -> "), point)(input)
}

pub(crate) fn paths(input: &str) -> IResult<&str, Vec<Path>> {
    separated_list0(newline, path)(input)
}
