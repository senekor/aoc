fn game(input: &str, limit: usize) -> usize {
    let initial_numbers = input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let n = initial_numbers.len();
    let mut num_turn = Vec::with_capacity(n + 1);
    for (i, init_num) in initial_numbers.into_iter().enumerate() {
        if num_turn.len() <= init_num {
            num_turn.resize(init_num + 1, 0);
        }
        num_turn[init_num] = i + 1;
    }
    let mut call = 0;
    for i in n + 1..limit {
        if num_turn.len() <= call {
            num_turn.resize(call + 1, 0);
        }
        if num_turn[call] == 0 {
            num_turn[call] = i;
            call = 0;
        } else {
            let prev = call;
            call = i - num_turn[prev];
            num_turn[prev] = i;
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
