use std::collections::HashSet;

fn find_start_marker<const OFFSET: usize>(input: &str) -> usize {
    input
        .as_bytes()
        .windows(OFFSET)
        .enumerate()
        .find(|(_, w)| w.iter().copied().collect::<HashSet<u8>>().len() == OFFSET)
        .unwrap()
        .0
        + OFFSET
}

pub fn part1(input: &str) -> usize {
    find_start_marker::<4>(input)
}

pub fn part2(input: &str) -> usize {
    find_start_marker::<14>(input)
}
