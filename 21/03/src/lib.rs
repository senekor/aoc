use utils::Itertools;

pub fn part1(input: &str) -> usize {
    let l = input.lines().next().unwrap().len();
    let n = input.lines().count();

    let mut v = vec![0; l];

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

    gamma * epsilon
}

fn apply_bit_criteria(diagnostic_report: &[&str], flipped: bool) -> usize {
    let mut candidates = Vec::from(diagnostic_report);
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
    usize::from_str_radix(candidates[0], 2).unwrap()
}

pub fn part2(input: &str) -> usize {
    let diagnostic_report = input.lines().collect_vec();
    let oxygen = apply_bit_criteria(&diagnostic_report, false);
    let co2 = apply_bit_criteria(&diagnostic_report, true);

    oxygen * co2
}
