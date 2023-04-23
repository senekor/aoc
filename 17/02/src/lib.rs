use utils::Itertools;

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap());
            nums.clone().max().unwrap() - nums.clone().min().unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap());
            nums.clone()
                .cartesian_product(nums)
                .find_map(|(n1, n2)| match () {
                    _ if n1 == n2 => None,
                    _ if n1 % n2 == 0 => Some(n1 / n2),
                    _ if n2 % n1 == 0 => Some(n2 / n1),
                    _ => None,
                })
                .unwrap()
        })
        .sum()
}
