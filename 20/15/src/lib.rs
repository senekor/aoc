use std::collections::HashMap;

fn game(input: &str, limit: usize) -> usize {
    let initial_numbers = input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let n = initial_numbers.len();
    let mut num_turn = HashMap::new();
    for (i, init_num) in initial_numbers.into_iter().enumerate() {
        num_turn.insert(init_num, i);
    }
    let mut call = 0;
    for i in n..limit - 1 {
        if !num_turn.contains_key(&call) {
            num_turn.insert(call, i);
            call = 0
        } else {
            let prev = call;
            call = i - num_turn[&prev];
            num_turn.insert(prev, i);
        }
    }
    call
}

pub fn part1(input: &str) -> usize {
    game(input, 2020)
}

pub fn part2(input: &str) -> usize {
    game(input, 30_000_000)
}
