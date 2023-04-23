use utils::Itertools;

fn find_mistake(line: &str) -> u8 {
    line.as_bytes()
        .iter()
        .copied()
        .find(|l| line[line.len() / 2..].as_bytes().contains(l))
        .expect("no mistake found")
}

fn get_priority(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c - b'A' + 27,
        b'a'..=b'z' => c - b'a' + 1,
        _ => panic!("unknown character {}", c),
    }
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| get_priority(find_mistake(line)) as u32)
        .sum()
}

fn find_badge(group: Vec<&str>) -> u8 {
    group[0]
        .as_bytes()
        .iter()
        .copied()
        .find(|l| group[1].as_bytes().contains(l) && group[2].as_bytes().contains(l))
        .expect("no mistake found")
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| get_priority(find_badge(group.collect())) as u32)
        .sum()
}
