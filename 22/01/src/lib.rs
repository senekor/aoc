use std::cmp::Reverse;

fn count_calories(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<u32>().unwrap()).sum())
}

pub fn part1(input: &str) -> u32 {
    count_calories(input).max().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let mut elfs: Vec<u32> = count_calories(input).collect();
    elfs.sort_by_key(|&elf| Reverse(elf));
    elfs.into_iter().take(3).sum()
}
