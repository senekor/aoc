use utils::Itertools;

mod parse;

type Integer = i32;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Int(Integer),
    List(Vec<Item>),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Item::*;
        match (self, other) {
            (Int(i_1), Int(i_2)) => i_1.cmp(i_2),
            (List(l_1), List(l_2)) => l_1.cmp(l_2),
            (Int(i), List(l)) => [Int(*i)].as_slice().cmp(l.as_slice()),
            (List(l), Int(i)) => l.as_slice().cmp(&[Int(*i)]),
        }
    }
}

type Packet = Vec<Item>;
type PacketPair = [Packet; 2];

pub fn part1(input: &str) -> usize {
    let (_, packet_pairs) = parse::packets(input).unwrap();
    packet_pairs
        .into_iter()
        .enumerate()
        .filter(|(_, [p_1, p_2])| p_1 <= p_2)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (_, packet_pairs) = parse::packets(input).unwrap();
    let mut packets = packet_pairs.into_iter().flatten().collect_vec();
    let divider_packets = [
        vec![Item::List(vec![Item::Int(2)])],
        vec![Item::List(vec![Item::Int(6)])],
    ];
    packets.extend_from_slice(&divider_packets);
    packets.sort();
    packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| divider_packets.contains(p))
        .map(|(i, _)| i + 1)
        .product()
}
