use itertools::*;

fn close_to_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unknown closing chunk: {}", c),
    }
}

fn is_close(c: char) -> bool {
    matches!(c, ')' | ']' | '}' | '>')
}

fn close_to_open(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("unknown closing chunk: {}", c),
    }
}

fn part1(input: Vec<String>) {
    let mut score = 0;
    for line in input {
        let mut stack = Vec::new();
        for c in line.chars() {
            if is_close(c) {
                if stack.is_empty() || stack[stack.len() - 1] != close_to_open(c) {
                    score += close_to_score(c);
                    break;
                } else {
                    stack.pop();
                }
            } else {
                stack.push(c);
            }
        }
    }

    println!("{:?}", score)
}

fn open_to_score_2(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("unknown closing chunk: {}", c),
    }
}

fn part2(input: Vec<String>) {
    let mut scores = Vec::new();
    for line in input {
        let mut corrupted = false;
        let mut stack = Vec::new();

        for c in line.chars() {
            if is_close(c) {
                if stack.is_empty() || stack[stack.len() - 1] != close_to_open(c) {
                    corrupted = true;
                    break;
                } else {
                    stack.pop();
                }
            } else {
                stack.push(c);
            }
        }

        if !corrupted {
            let mut score = 0;
            // println!("{:#?}", stack);
            while let Some(c) = stack.pop() {
                // println!("{}", score);
                score = score * 5 + open_to_score_2(c)
            }
            scores.push(score);
        }
    }

    scores.sort_unstable();
    println!("{} {}", scores.len(), scores.len() / 2 + 1);

    println!("{:?}", scores[scores.len() / 2])
}

fn main() {
    let input = include_str!("../input/input.txt")
        .lines()
        .map(|line| line.to_owned())
        .collect_vec();

    part1(input.clone());

    part2(input);
}
