fn run(num_players: usize, last_marble: usize) -> usize {
    let mut players = vec![0; num_players];

    let mut current_location = 0;
    let mut marbles = vec![0];
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            let current_player = &mut players[marble % num_players];
            *current_player += marble;
            current_location = (current_location + marbles.len() - 7) % marbles.len();
            *current_player += marbles.remove(current_location);
            continue;
        }
        current_location = (current_location + 1) % marbles.len() + 1;
        marbles.insert(current_location, marble);
    }
    players.into_iter().max().unwrap()
}

pub fn part1(input: &str) -> usize {
    let mut tokens = input.split_whitespace();
    let num_players: usize = tokens.next().unwrap().parse().unwrap();
    let last_marble: usize = tokens.nth(5).unwrap().parse().unwrap();

    run(num_players, last_marble)
}

pub fn part2(input: &str) -> usize {
    let mut tokens = input.split_whitespace();
    let num_players: usize = tokens.next().unwrap().parse().unwrap();
    let last_marble: usize = tokens.nth(5).unwrap().parse().unwrap();

    run(num_players, last_marble * 100)
}
