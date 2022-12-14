use itertools::*;

type Board = [[(i32, bool); 5]; 5];

fn calc_score(board: &mut Board, last_num: i32) -> i32 {
    let mut unmarked_sum = 0;
    for row in board {
        for cell in row {
            if !cell.1 {
                unmarked_sum += cell.0;
            }
        }
    }
    unmarked_sum * last_num
}

fn calc_maybe_score(board: &mut Board, last_num: i32) -> Option<i32> {
    let mut bingo = false;

    // rows & cols
    for i in 0..5 {
        let mut count_row = 0;
        let mut count_col = 0;
        for j in 0..5 {
            if board[i][j].1 {
                count_row += 1;
            }
            if board[j][i].1 {
                count_col += 1;
            }
        }
        if count_row == 5 || count_col == 5 {
            bingo = true;
        }
    }

    if bingo {
        return Some(calc_score(board, last_num));
    }

    None
}

pub fn part1(input: &str) -> i32 {
    let squid_said_all: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect_vec();

    let mut boards: Vec<Board> = Vec::new();

    let boards_input = input.split("\n\n").skip(1);
    for board_input in boards_input {
        let mut new_board = [[(0, false); 5]; 5];
        for (i, row) in board_input.lines().enumerate() {
            for (j, cell) in row.trim().split_ascii_whitespace().enumerate() {
                new_board[i][j].0 = cell.parse().unwrap();
            }
        }
        boards.push(new_board);
    }

    for squid_said in squid_said_all {
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for cell in row {
                    if cell.0 == squid_said {
                        cell.1 = true;
                    }
                }
            }
            let maybe_score = calc_maybe_score(board, squid_said);
            if let Some(final_score) = maybe_score {
                return final_score;
            }
        }
    }
    panic!()
}

pub fn part2(input: &str) -> i32 {
    let squid_said_all: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect_vec();

    let mut boards: Vec<(Board, bool)> = Vec::new();

    let boards_input = input.split("\n\n").skip(1);
    for board_input in boards_input {
        let mut new_board = [[(0, false); 5]; 5];
        for (i, row) in board_input.lines().enumerate() {
            for (j, cell) in row.trim().split_ascii_whitespace().enumerate() {
                new_board[i][j].0 = cell.parse().unwrap();
            }
        }
        boards.push((new_board, false));
    }

    let mut all_scores = Vec::new();

    for squid_said in squid_said_all {
        for board in boards.iter_mut() {
            if board.1 {
                continue;
            };
            for row in board.0.iter_mut() {
                for cell in row {
                    if cell.0 == squid_said {
                        cell.1 = true;
                    }
                }
            }
            let maybe_score = calc_maybe_score(&mut board.0, squid_said);
            if let Some(final_score) = maybe_score {
                board.1 = true;
                all_scores.push(final_score);
            }
        }
    }

    all_scores[all_scores.len() - 1]
}
