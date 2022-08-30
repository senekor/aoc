use itertools::*;

pub fn part1(input: &str) -> u32 {
    let entries = input.lines().map(|line| line.parse::<u32>().unwrap());
    for comb in entries.clone().cartesian_product(entries) {
        if comb.0 + comb.1 == 2020 {
            return comb.0 * comb.1;
        }
    }
    panic!()
}

pub fn part2(input: &str) -> u32 {
    let entries = input.lines().map(|line| line.parse::<u32>().unwrap());
    for comb in entries
        .clone()
        .cartesian_product(entries.clone())
        .cartesian_product(entries)
    {
        if comb.0 .0 + comb.0 .1 + comb.1 == 2020 {
            return comb.0 .0 * comb.0 .1 * comb.1;
        }
    }
    panic!()
}
