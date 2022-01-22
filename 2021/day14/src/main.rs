use std::collections::{hash_map::Entry, HashMap};

use itertools::*;

type Line = (String, String);
fn parse(line: &str) -> Line {
    {
        let mut iter = line.split(" -> ");
        (
            iter.next().unwrap().parse::<String>().unwrap(),
            iter.next().unwrap().parse::<String>().unwrap(),
        )
    }
}

fn part1(start: String, instructions: HashMap<String, String>) {
    let mut current = start;
    for _ in 0..10 {
        let mut next = current.clone();
        for i in (0..current.len() - 1).rev() {
            let tuple = current[i..i + 2].to_string();
            if instructions.contains_key(&tuple) {
                let insert = instructions.get(&tuple).unwrap();
                next = next[..i + 1].to_string() + insert + &next[i + 1..];
            }
        }
        current = next;
        // println!("{:?}", current);
    }

    let occurances = current
        .chars()
        .fold(HashMap::<char, usize>::new(), |mut m, c| {
            if let Entry::Vacant(e) = m.entry(c) {
                e.insert(0);
            } else {
                m.entry(c).and_modify(|e| *e += 1);
            };
            m
        });

    let mut min = usize::MAX;
    let mut max = 0;
    for (_, val) in occurances {
        if val < min {
            min = val;
        };
        if val > max {
            max = val;
        };
    }

    println!("{:?}", max - min);
}

fn part2(start: String, instructions: HashMap<String, String>) {
    let mut current = {
        let mut m: HashMap<String, usize> = HashMap::new();
        for i in 0..start.len() - 1 {
            if m.contains_key(&start[i..i + 2]) {
                m.entry(start[i..i + 2].to_string()).and_modify(|e| *e += 1);
            } else {
                m.insert(start[i..i + 2].to_string(), 1);
            };
        }
        m
    };
    for _ in 0..40 {
        let mut next = HashMap::new();
        for (tuple, count) in current {
            if instructions.contains_key(&tuple) {
                let insert = instructions.get(&tuple).unwrap();
                let left = tuple[..1].to_string() + insert;
                let right = insert.to_string() + &tuple[1..];
                if let Entry::Vacant(e) = next.entry(left.clone()) {
                    e.insert(count);
                } else {
                    next.entry(left).and_modify(|e| *e += count);
                };
                if let Entry::Vacant(e) = next.entry(right.clone()) {
                    e.insert(count);
                } else {
                    next.entry(right).and_modify(|e| *e += count);
                };
            } else if let Entry::Vacant(e) = next.entry(tuple.clone()) {
                e.insert(count);
            } else {
                next.entry(tuple).and_modify(|e| *e += count);
            }
        }
        current = next;
        // println!("{:#?}", current);
    }

    let mut occurances = HashMap::new();
    for (tuple, count) in current {
        for c in tuple.chars() {
            if let Entry::Vacant(e) = occurances.entry(c) {
                e.insert(count);
            } else {
                occurances.entry(c).and_modify(|e| *e += count);
            };
        }
    }

    let mut min = usize::MAX;
    let mut max = 0;
    for (_, val) in occurances {
        if val < min {
            min = val;
        };
        if val > max {
            max = val;
        };
    }

    println!("{:?}", (max - min) / 2)
}

fn main() {
    let input = include_str!("../input/input.txt");
    let mut bruh = input.split("\n\n");
    let start = bruh.next().unwrap().to_string();

    let instructions = bruh.next().unwrap().lines().map(parse).collect_vec();
    let instructions = {
        let mut m = HashMap::new();
        for instr in instructions {
            m.insert(instr.0, instr.1);
        }
        m
    };

    part1(start.clone(), instructions.clone());

    part2(start, instructions);
}
