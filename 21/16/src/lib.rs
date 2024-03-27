fn hex_to_bits(hex_str: &str) -> Vec<u8> {
    hex_str
        .chars()
        .flat_map(|c| {
            let hex_num = u8::from_str_radix(c.to_string().as_str(), 16).unwrap();
            [
                (hex_num >> 3) % 2,
                (hex_num >> 2) % 2,
                (hex_num >> 1) % 2,
                hex_num % 2,
            ]
            .into_iter()
        })
        .collect::<Vec<_>>()
}

fn parse_header(bits: &[u8]) -> (u8, u8, &[u8]) {
    let version = (bits[0] << 2) + (bits[1] << 1) + bits[2];
    let type_id = (bits[3] << 2) + (bits[4] << 1) + bits[5];
    (version, type_id, &bits[6..])
}

#[derive(Debug)]
enum Packet {
    OpPacket {
        version: u8,
        type_id: u8,
        children: Vec<Packet>,
    },
    LitPacket {
        version: u8,
        _type_id: u8,
        val: usize,
    },
}
use Packet::*;

fn parse_literal(payload: &[u8]) -> (usize, usize) {
    let mut i = 0;
    let mut literal = 0;
    loop {
        literal <<= 4;
        literal += ((payload[i + 1] as usize) << 3)
            + ((payload[i + 2] as usize) << 2)
            + ((payload[i + 3] as usize) << 1)
            + (payload[i + 4] as usize);

        if payload[i] == 0 {
            break;
        }

        i += 5;
    }
    (literal, i + 5)
}

fn parse_children_length(bits: &[u8], len: usize) -> Vec<Packet> {
    let mut children = Vec::new();
    let mut len_parsed = 0;
    while len_parsed != len {
        let (child, child_len) = parse_packet(&bits[len_parsed..]);
        children.push(child);
        len_parsed += child_len;
    }
    children
}

fn parse_children_num(bits: &[u8], num: usize) -> (Vec<Packet>, usize) {
    let mut children = Vec::new();
    let mut len_parsed = 0;
    for _ in 0..num {
        let (child, len) = parse_packet(&bits[len_parsed..]);
        children.push(child);
        len_parsed += len;
    }
    (children, len_parsed)
}

fn parse_children(payload: &[u8]) -> (Vec<Packet>, usize) {
    if payload[0] == 0 {
        let mut total_len_subpackets = 0;
        for (i, b) in payload[1..16].iter().rev().enumerate() {
            total_len_subpackets += (*b as usize) << i;
        }
        (
            parse_children_length(&payload[16..], total_len_subpackets),
            total_len_subpackets + 16,
        )
    } else {
        let mut total_num_subpackets = 0;
        for (i, b) in payload[1..12].iter().rev().enumerate() {
            total_num_subpackets += (*b as usize) << i;
        }
        let (children, len_children) = parse_children_num(&payload[12..], total_num_subpackets);
        (children, len_children + 12)
    }
}

fn parse_packet(bits: &[u8]) -> (Packet, usize) {
    let (version, type_id, payload) = parse_header(bits);
    if type_id == 4 {
        let (val, len) = parse_literal(payload);
        return (
            LitPacket {
                version,
                _type_id: type_id,
                val,
            },
            len + 6,
        );
    }
    let (children, len) = parse_children(payload);
    (
        OpPacket {
            version,
            type_id,
            children,
        },
        len + 6,
    )
}

fn sum_versions(packet: &Packet) -> usize {
    match packet {
        OpPacket {
            version,
            type_id: _,
            children,
        } => (*version as usize) + children.iter().map(sum_versions).sum::<usize>(),
        LitPacket {
            version,
            _type_id: _,
            val: _,
        } => *version as usize,
    }
}

pub fn part1(input: &str) -> usize {
    let bits = hex_to_bits(input);
    let (packet, _) = parse_packet(&bits);

    sum_versions(&packet)
}

fn execute_op(op: u8, packets: &[Packet]) -> usize {
    match op {
        0 => packets.iter().map(execute).sum(),
        1 => packets
            .iter()
            .map(execute)
            .reduce(|acc, next| acc * next)
            .unwrap(),
        2 => packets.iter().map(execute).min().unwrap(),
        3 => packets.iter().map(execute).max().unwrap(),
        5 => usize::from(execute(&packets[0]) > execute(&packets[1])),
        6 => usize::from(execute(&packets[0]) < execute(&packets[1])),
        7 => usize::from(execute(&packets[0]) == execute(&packets[1])),
        x => panic!("did not expect op: {}", x),
    }
}

fn execute(packet: &Packet) -> usize {
    match packet {
        OpPacket {
            version: _,
            type_id,
            children,
        } => execute_op(*type_id, children),
        LitPacket {
            version: _,
            _type_id: _,
            val,
        } => *val,
    }
}

pub fn part2(input: &str) -> usize {
    let bits = hex_to_bits(input);
    let (packet, _) = parse_packet(&bits);

    execute(&packet)
}
