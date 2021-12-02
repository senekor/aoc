use itertools::*;

fn part1(input: &str) {
    let l = input.lines().next().unwrap().len();
    let n = input.lines().count();

    let mut v = Vec::with_capacity(l);
    v.resize(l, 0);

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                v[i] += 1;
            };
        }
    }

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    for (i, num_occ) in v.into_iter().rev().enumerate() {
        if num_occ > n / 2 {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }

    println!("{:?}", gamma * epsilon)
}

fn part2(input: &str) {
    let l = input.lines().next().unwrap().len();

    let mut v = Vec::with_capacity(l);
    v.resize(l, 0);

    let mut candidates = input.lines().collect_vec();
    for i in 0..l {
        let count = candidates
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .filter(|c| *c == '1')
            .count();
        let n = candidates.len();
        let req_char = {
            if count > n / 2 {
                '1'
            } else if count == n / 2 && n % 2 == 0 {
                '1'
            } else {
                '0'
            }
        };
        candidates = candidates
            .into_iter()
            .filter(|line| {
                let relevant = line.chars().nth(i).unwrap();
                relevant == req_char
            })
            .collect_vec();

        if candidates.len() == 1 {
            break;
        }
    }
    let oxygen = usize::from_str_radix(candidates[0], 2).unwrap();

    let mut candidates = input.lines().collect_vec();
    for i in 0..l {
        let count = candidates
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .filter(|c| *c == '1')
            .count();
        let n = candidates.len();
        let req_char = {
            if count > n / 2 {
                '0'
            } else if count == n / 2 && n % 2 == 0 {
                '0'
            } else {
                '1'
            }
        };
        candidates = candidates
            .into_iter()
            .filter(|line| {
                let relevant = line.chars().nth(i).unwrap();
                relevant == req_char
            })
            .collect_vec();

        if candidates.len() == 3 {
            println!("{:?}", candidates)
        }
        if candidates.len() == 1 {
            break;
        }
    }
    let co2 = usize::from_str_radix(candidates[0], 2).unwrap();

    println!("{:?}", oxygen * co2);
}

fn main() {
    let input = include_str!("../input/input.txt");

    part1(&input);

    part2(&input);
}
