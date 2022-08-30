use itertools::Itertools;

const VOLUME: i32 = 150;

fn part1(input: &str) {
    let containers: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    let res = (1..containers.len())
        .flat_map(|i| containers.iter().combinations(i))
        .filter(|combination| combination.iter().copied().sum::<i32>() == VOLUME)
        .count();

    println!("number of possible combinations: {}", res);
}

fn part2(input: &str) {
    let containers: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    let res = (1..containers.len())
        .flat_map(|i| containers.iter().combinations(i))
        .filter(|combination| combination.iter().copied().sum::<i32>() == VOLUME)
        .map(|combination| combination.len())
        .fold((containers.len(), 0), |acc, len| match () {
            _ if len < acc.0 => (len, 1),
            _ if len == acc.0 => (acc.0, acc.1 + 1),
            _ => acc,
        })
        .1;

    println!("number of ways to fill minimal container count: {}", res);
}

fn main() {
    let input = include_str!("../input/input.txt");
    part1(input);
    part2(input);
}
