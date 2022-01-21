use itertools::*;

fn part1(input: &str) {
    let l = input.lines().next().unwrap().len();
    let n = input.lines().count();

    let mut v = Vec::with_capacity(l);
    v.resize(l, 0);

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            v[i] += c.to_string().parse::<usize>().unwrap();
        }
    }

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    for (i, num_ones) in v.into_iter().rev().enumerate() {
        if num_ones > n / 2 {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }

    println!("{:?}", gamma * epsilon) // 4174964
}

fn apply_bit_criteria(diagnostic_report: &Vec<&str>, flipped: bool) -> usize {
    let mut candidates = diagnostic_report.clone();
    let l = candidates[0].len();
    for i in 0..l {
        let num_ones_at_i = candidates
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .filter(|c| *c == '1')
            .count();
        let n = candidates.len();
        let more_ones_or_equal = num_ones_at_i >= (n - num_ones_at_i);
        let req_char = match more_ones_or_equal != flipped {
            true => '1',
            false => '0',
        };
        candidates = candidates
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == req_char)
            .collect_vec();
        if candidates.len() == 1 {
            break;
        }
    }
    return usize::from_str_radix(candidates[0], 2).unwrap();
}

fn part2(input: &str) {
    let diagnostic_report = input.lines().collect_vec();
    let oxygen = apply_bit_criteria(&diagnostic_report, false);
    let co2 = apply_bit_criteria(&diagnostic_report, true);

    println!("{:?}", oxygen * co2) // 4474944
}

fn main() {
    let input = include_str!("../input/input.txt");

    part1(&input);

    part2(&input);
}
