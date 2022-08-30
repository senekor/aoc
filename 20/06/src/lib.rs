use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(char::is_ascii_alphabetic)
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.chars().collect::<HashSet<_>>())
                .reduce(|acc, next| acc.intersection(&next).copied().collect())
                .unwrap()
                .len()
        })
        .sum()
}
