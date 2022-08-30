use std::collections::{HashMap, HashSet};

fn right(pos: &mut (i32, i32)) {
    pos.0 += 1;
}
fn left(pos: &mut (i32, i32)) {
    pos.0 -= 1;
}
fn up(pos: &mut (i32, i32)) {
    pos.1 += 1;
}
fn down(pos: &mut (i32, i32)) {
    pos.1 -= 1;
}

pub fn part1(input: &str) -> i32 {
    let mut iter = input.lines();
    let wire_1 = iter.next().unwrap();
    let mut pos = (0, 0);
    let mut visited = HashSet::new();
    for instr in wire_1.split(',') {
        let mutate = match instr.get(0..1).unwrap() {
            "R" => right,
            "L" => left,
            "U" => up,
            "D" => down,
            d => panic!("unknown direction {d}"),
        };
        let dist = instr.get(1..).unwrap().parse::<u32>().unwrap();
        for _ in 0..dist {
            mutate(&mut pos);
            visited.insert(pos);
        }
    }

    let wire_2 = iter.next().unwrap();
    pos = (0, 0);
    let mut min_dist = i32::MAX;
    for instr in wire_2.split(',') {
        let mutate = match instr.get(0..1).unwrap() {
            "R" => right,
            "L" => left,
            "U" => up,
            "D" => down,
            d => panic!("unknown direction {d}"),
        };
        let dist = instr.get(1..).unwrap().parse::<u32>().unwrap();
        for _ in 0..dist {
            mutate(&mut pos);
            if visited.contains(&pos) {
                let taxi_cab_dist = pos.0.abs() + pos.1.abs();
                if taxi_cab_dist < min_dist {
                    min_dist = taxi_cab_dist;
                }
            }
        }
    }

    min_dist
}

pub fn part2(input: &str) -> i32 {
    let mut iter = input.lines();
    let wire_1 = iter.next().unwrap();
    let mut pos = (0, 0);
    let mut wire_len = 0;
    let mut visited = HashMap::new();
    for instr in wire_1.split(',') {
        let mutate = match instr.get(0..1).unwrap() {
            "R" => right,
            "L" => left,
            "U" => up,
            "D" => down,
            d => panic!("unknown direction {d}"),
        };
        let dist = instr.get(1..).unwrap().parse::<u32>().unwrap();
        for _ in 0..dist {
            mutate(&mut pos);
            wire_len += 1;
            visited.insert(pos, wire_len);
        }
    }

    let wire_2 = iter.next().unwrap();
    pos = (0, 0);
    wire_len = 0;
    let mut min_dist = i32::MAX;
    for instr in wire_2.split(',') {
        let mutate = match instr.get(0..1).unwrap() {
            "R" => right,
            "L" => left,
            "U" => up,
            "D" => down,
            d => panic!("unknown direction {d}"),
        };
        let dist = instr.get(1..).unwrap().parse::<u32>().unwrap();
        for _ in 0..dist {
            mutate(&mut pos);
            wire_len += 1;
            if let Some(&len) = visited.get(&pos) {
                let both_wire_len = wire_len + len;
                if both_wire_len < min_dist {
                    min_dist = both_wire_len;
                }
            }
        }
    }

    min_dist
}
