use std::cmp::{max, min};
use std::str::FromStr;

use utils::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Block {
    is_26: bool,
    param2: i32,
    param3: i32,
}

impl FromStr for Block {
    type Err = String;
    fn from_str(block: &str) -> Result<Self, Self::Err> {
        let mut lines = block.lines();
        Ok(Block {
            is_26: 26
                == lines
                    .nth(3)
                    .unwrap()
                    .split(' ')
                    .nth(2)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
            param2: lines
                .next()
                .unwrap()
                .split(' ')
                .nth(2)
                .unwrap()
                .parse()
                .unwrap(),
            param3: lines
                .nth(9)
                .unwrap()
                .split(' ')
                .nth(2)
                .unwrap()
                .parse()
                .unwrap(),
        })
    }
}

fn parse_prog(input: &str) -> Vec<Block> {
    input
        .split("inp w\n")
        .skip(1)
        .map(|block| block.parse().unwrap())
        .collect_vec()
}

pub fn part1(input: &str) -> String {
    let prog = parse_prog(input);
    let mut stack: Vec<(usize, &Block)> = Vec::new();
    let mut digits = vec![0; prog.len()];
    for (i, block) in prog.iter().enumerate() {
        if block.is_26 {
            let (other_i, block_from_stack) = stack.pop().unwrap();
            let param_sum = block.param2 + block_from_stack.param3;
            digits[i] = min(9, 9 + param_sum);
            digits[other_i] = min(9, 9 - param_sum);
        } else {
            stack.push((i, block))
        }
    }

    digits
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>()
}

pub fn part2(input: &str) -> String {
    let prog = parse_prog(input);
    let mut stack: Vec<(usize, &Block)> = Vec::new();
    let mut digits = vec![0; prog.len()];
    for (i, block) in prog.iter().enumerate() {
        if block.is_26 {
            let (other_i, block_from_stack) = stack.pop().unwrap();
            let param_sum = block.param2 + block_from_stack.param3;
            digits[i] = max(1, 1 + param_sum);
            digits[other_i] = max(1, 1 - param_sum);
        } else {
            stack.push((i, block))
        }
    }

    digits
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>()
}
