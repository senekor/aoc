use utils::nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::{Integer, Item, Packet, PacketPair};

fn integer(input: &str) -> IResult<&str, Integer> {
    map(digit1, |digs: &str| digs.parse().unwrap())(input)
}

fn list(input: &str) -> IResult<&str, Vec<Item>> {
    delimited(char('['), separated_list0(char(','), item), char(']'))(input)
}

fn item(input: &str) -> IResult<&str, Item> {
    alt((map(integer, Item::Int), map(list, Item::List)))(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    list(input)
}

fn packet_pair(input: &str) -> IResult<&str, PacketPair> {
    map(separated_pair(packet, char('\n'), packet), |(a, b)| [a, b])(input)
}

pub(crate) fn packets(input: &str) -> IResult<&str, Vec<PacketPair>> {
    separated_list1(tag("\n\n"), packet_pair)(input)
}
