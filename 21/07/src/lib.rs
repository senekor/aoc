fn calc_fuel_consumption(crabs: &[i32], alignment_pos: i32) -> i32 {
    crabs.iter().map(|crab| (crab - alignment_pos).abs()).sum()
}

fn median_crab(crabs: &[i32]) -> i32 {
    let mut clone = crabs.to_owned();
    clone.sort_unstable();
    clone[clone.len() / 2]
}

pub fn part1(input: &str) -> i32 {
    let crabs = input
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
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
    calc_fuel_consumption(&crabs, bruh)
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

pub fn part2(input: &str) -> i32 {
    let crabs = input
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
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
    calc_fuel_consumption_2(&crabs, bruh)
}
