use itertools::*;

fn calc_fuel_consumption(crabs: &[i32], alignment_pos: i32) -> i32 {
    crabs.iter().map(|crab| (crab - alignment_pos).abs()).sum()
}

fn median_crab(crabs: &[i32]) -> i32 {
    let mut clone = crabs.to_owned();
    clone.sort_unstable();
    clone[clone.len() / 2]
}

fn part1(crabs: Vec<i32>) {
    let mut bruh = median_crab(&crabs);
    loop {
        let prev = calc_fuel_consumption(&crabs, bruh);
        let next = calc_fuel_consumption(&crabs, bruh + 1);
        if next <= prev {
            bruh += 1
        } else {
            break;
        }
    }
    loop {
        let prev = calc_fuel_consumption(&crabs, bruh);
        let next = calc_fuel_consumption(&crabs, bruh - 1);
        if next <= prev {
            bruh -= 1
        } else {
            break;
        }
    }
    println!("{:?}", calc_fuel_consumption(&crabs, bruh))
}

fn calc_fuel_consumption_2(crabs: &[i32], alignment_pos: i32) -> i32 {
    crabs
        .iter()
        .map(|crab| {
            let diff = (crab - alignment_pos).abs();
            if diff > 0 {
                (1..=diff).sum()
            } else {
                0
            }
        })
        .sum()
}

fn part2(crabs: Vec<i32>) {
    let mut bruh = median_crab(&crabs);
    loop {
        let prev = calc_fuel_consumption_2(&crabs, bruh);
        let next = calc_fuel_consumption_2(&crabs, bruh + 1);
        if next <= prev {
            bruh += 1
        } else {
            break;
        }
    }
    loop {
        let prev = calc_fuel_consumption_2(&crabs, bruh);
        let next = calc_fuel_consumption_2(&crabs, bruh - 1);
        if next <= prev {
            bruh -= 1
        } else {
            break;
        }
    }
    println!("{:?}", calc_fuel_consumption_2(&crabs, bruh))
}

fn main() {
    let input = include_str!("../input/input.txt")
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect_vec();

    part1(input.clone());

    part2(input);
}
