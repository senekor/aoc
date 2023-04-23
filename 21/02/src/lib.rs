use std::{panic, str::FromStr};
use utils::Itertools;

#[derive(Clone, Debug)]
enum Direction {
    Forw,
    Up,
    Down,
}
use Direction::*;

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "forward" => Forw,
            "up" => Up,
            "down" => Down,
            x => panic!("unknown direction {}", x),
        })
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    dir: Direction,
    num: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        Ok(Instruction {
            dir: iter.next().unwrap().parse().unwrap(),
            num: iter.next().unwrap().parse().unwrap(),
        })
    }
}

struct Instructions(Vec<Instruction>);

pub fn part1(input: &str) -> i32 {
    let instructions = Instructions(
        input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect_vec(),
    );

    let mut depth = 0;
    let mut horiz = 0;

    for instr in &instructions.0 {
        match instr.dir {
            Forw => horiz += instr.num,
            Up => depth -= instr.num,
            Down => depth += instr.num,
        }
    }

    depth * horiz
}

pub fn part2(input: &str) -> i32 {
    let instructions = Instructions(
        input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect_vec(),
    );

    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;

    for instr in &instructions.0 {
        match instr.dir {
            Forw => {
                horiz += instr.num;
                depth += aim * instr.num;
            }
            Up => aim -= instr.num,
            Down => aim += instr.num,
        }
    }

    depth * horiz
}
