use winnow::{
    ascii::digit1,
    combinator::{alt, delimited, separated, separated_pair},
    PResult, Parser,
};

use crate::{Integer, Item, Packet, PacketPair};

fn integer(input: &mut &str) -> PResult<Integer> {
    digit1
        .map(|digs: &str| digs.parse().unwrap())
        .parse_next(input)
}

fn list(input: &mut &str) -> PResult<Vec<Item>> {
    delimited('[', separated(.., item, ','), ']').parse_next(input)
}

fn item(input: &mut &str) -> PResult<Item> {
    alt((integer.map(Item::Int), list.map(Item::List))).parse_next(input)
}

fn packet(input: &mut &str) -> PResult<Packet> {
    list(input)
}

fn packet_pair(input: &mut &str) -> PResult<PacketPair> {
    separated_pair(packet, '\n', packet)
        .map(|(a, b)| [a, b])
        .parse_next(input)
}

pub(crate) fn packets(input: &mut &str) -> PResult<Vec<PacketPair>> {
    separated(1.., packet_pair, "\n\n").parse_next(input)
}
