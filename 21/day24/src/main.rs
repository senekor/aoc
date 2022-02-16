use std::cmp::{max, min};
use std::str::FromStr;

use itertools::*;

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
                    .parse()
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

fn part1(prog: Vec<Block>) {
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

    let res = digits
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>();
    println!("{}", res)
}

fn part2(prog: Vec<Block>) {
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

    let res = digits
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>();
    println!("{}", res)
}

fn main() {
    let input = include_str!("../input/input.txt");
    let prog = input
        .split("inp w\n")
        .skip(1)
        .map(|block| block.parse().unwrap())
        .collect_vec();

    part1(prog.clone()); // 51983999947999

    part2(prog); // 11211791111365
}
