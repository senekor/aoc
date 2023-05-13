use std::collections::VecDeque;

pub fn part1_impl(input: &str, preamble_size: usize) -> i64 {
    let mut preamble = VecDeque::new();
    preamble.reserve(preamble_size);
    for num in input.lines().map(|s| s.parse::<i64>().unwrap()) {
        if preamble.len() < preamble_size {
            preamble.push_back(num);
            continue;
        }

        if preamble.iter().all(|n| !preamble.contains(&(num - n))) {
            return num;
        }

        preamble.pop_front();
        preamble.push_back(num);
    }

    panic!("not found")
}

pub fn part1(input: &str) -> i64 {
    part1_impl(input, 25)
}

pub fn part2_impl(input: &str, preamble_size: usize) -> i64 {
    let target = part1_impl(input, preamble_size);
    let nums: Vec<i64> = input.lines().map(|s| s.parse::<i64>().unwrap()).collect();

    let (mut i, mut j) = (0, 1);

    while j <= nums.len() {
        let range = &nums[i..j];
        let contig_sum = range.iter().sum::<i64>();

        if contig_sum == target {
            return range.iter().min().unwrap() + range.iter().max().unwrap();
        }

        match () {
            _ if contig_sum == target => return nums[i] + nums[j - 1],
            _ if contig_sum < target => j += 1,
            _ if i + 1 == j => (i, j) = (i + 1, j + 1),
            _ => i += 1,
        }
    }

    panic!("not found")
}

pub fn part2(input: &str) -> i64 {
    part2_impl(input, 25)
}
