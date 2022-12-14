use std::{collections::HashMap, hash::Hash};

use itertools::*;

type Line = (String, i32);
fn parse_line(line: &str) -> Line {
    {
        let mut iter = line.split("position: ");
        (
            iter.next().unwrap().parse::<String>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        )
    }
}

fn parse_input(input: &str) -> Vec<(String, i32)> {
    input.lines().map(parse_line).collect_vec()
}

struct Die {
    x: i32,
    roll_count: i32,
}

impl Die {
    fn new() -> Die {
        Die {
            x: 0,
            roll_count: 0,
        }
    }

    fn roll(&mut self) -> i32 {
        self.x = (self.x % 100) + 1;
        self.roll_count += 1;
        self.x
    }
}

fn calc_next_tile(start: i32, num_steps: i32) -> i32 {
    ((start + num_steps - 1) % 10) + 1
}

enum TurnResult {
    Win,
    NoWin,
}
use TurnResult::*;

#[derive(Clone, Copy, Debug)]
struct Player {
    pos: i32,
    score: i32,
}

impl Player {
    fn new(starting_pos: i32) -> Player {
        Player {
            pos: starting_pos,
            score: 0,
        }
    }

    fn take_steps(&mut self, num_steps: i32) {
        self.pos = calc_next_tile(self.pos, num_steps);
    }

    fn take_turn(&mut self, die: &mut Die) -> TurnResult {
        self.take_steps(die.roll());
        self.take_steps(die.roll());
        self.take_steps(die.roll());
        self.score += self.pos;
        if self.score >= 1000 {
            Win
        } else {
            NoWin
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let input = parse_input(input);

    let mut player_1 = Player::new(input[0].1);
    let mut player_2 = Player::new(input[1].1);

    let mut die = Die::new();

    loop {
        if let Win = player_1.take_turn(&mut die) {
            return player_2.score * die.roll_count;
        }

        if let Win = player_2.take_turn(&mut die) {
            return player_1.score * die.roll_count;
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct QuantumPlayer {
    pos: i32,
    score: i32,
}

impl QuantumPlayer {
    fn new(starting_pos: i32) -> QuantumPlayer {
        QuantumPlayer {
            pos: starting_pos,
            score: 0,
        }
    }

    fn take_steps(mut self, num_steps: i32) -> QuantumPlayer {
        self.pos = calc_next_tile(self.pos, num_steps);
        self
    }

    fn take_turn(self) -> Vec<(QuantumPlayer, usize)> {
        let mut res = vec![
            (self.take_steps(3), 1),
            (self.take_steps(4), 3),
            (self.take_steps(5), 6),
            (self.take_steps(6), 7),
            (self.take_steps(7), 6),
            (self.take_steps(8), 3),
            (self.take_steps(9), 1),
        ];

        for (player, _) in res.iter_mut() {
            player.score += player.pos
        }
        res
    }

    fn has_won(&self) -> bool {
        self.score >= 21
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    player_1: QuantumPlayer,
    player_2: QuantumPlayer,
}

#[derive(Debug, Clone, Copy)]
struct Wins {
    player_1: usize,
    player_2: usize,
}

fn get_wins(state: State, known_states: &mut HashMap<State, Wins>) -> Wins {
    if let Some(&wins) = known_states.get(&state) {
        return wins;
    }

    let new_players_one = state.player_1.take_turn();
    let new_players_two = state.player_2.take_turn();

    let mut wins = Wins {
        player_1: 0,
        player_2: 0,
    };
    for (player_1, num_p_1) in new_players_one {
        if player_1.has_won() {
            wins.player_1 += num_p_1;
            continue;
        }
        for (player_2, num_p_2) in new_players_two.iter() {
            if player_2.has_won() {
                wins.player_2 += num_p_2;
                continue;
            }
            let sub_state = State {
                player_1,
                player_2: *player_2,
            };
            let sub_wins = get_wins(sub_state, known_states);
            wins.player_1 += sub_wins.player_1 * num_p_1 * num_p_2;
            wins.player_2 += sub_wins.player_2 * num_p_1 * num_p_2;
        }
    }
    known_states.insert(state, wins);
    wins
}

pub fn part2(input: &str) -> usize {
    let input = parse_input(input);

    let player_1 = QuantumPlayer::new(input[0].1);
    let player_2 = QuantumPlayer::new(input[1].1);

    let state = State { player_1, player_2 };
    let mut known_states = HashMap::new();
    let wins = get_wins(state, &mut known_states);
    wins.player_1.max(wins.player_2)
}
