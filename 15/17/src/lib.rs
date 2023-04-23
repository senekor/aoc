use utils::Itertools;

const VOLUME: i32 = 150;

pub fn part1(input: &str) -> usize {
    let containers: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    (1..containers.len())
        .flat_map(|i| containers.iter().combinations(i))
        .filter(|combination| combination.iter().copied().sum::<i32>() == VOLUME)
        .count()
}

pub fn part2(input: &str) -> i32 {
    let containers: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    (1..containers.len())
        .flat_map(|i| containers.iter().combinations(i))
        .filter(|combination| combination.iter().copied().sum::<i32>() == VOLUME)
        .map(|combination| combination.len())
        .fold((containers.len(), 0), |acc, len| match () {
            _ if len < acc.0 => (len, 1),
            _ if len == acc.0 => (acc.0, acc.1 + 1),
            _ => acc,
        })
        .1
}
