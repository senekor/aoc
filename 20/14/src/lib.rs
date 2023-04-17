use std::{collections::HashMap, iter::once};

fn prepare_masks(val: &str) -> (u64, u64) {
    let (mut and_mask, mut or_mask) = (0, 0);
    for (i, ch) in val.chars().rev().enumerate() {
        if ch == '0' {
            and_mask |= 1 << i
        } else if ch == '1' {
            or_mask |= 1 << i
        }
    }
    and_mask = !and_mask;
    (and_mask, or_mask)
}

fn apply(masks: (u64, u64), val: &str) -> u64 {
    let (and_mask, or_mask) = masks;
    let mut final_val = val.parse::<u64>().unwrap();
    final_val &= and_mask;
    final_val |= or_mask;
    final_val
}

fn exec_mem_instr(mem: &mut HashMap<usize, u64>, masks: (u64, u64), instr: &str, val: &str) {
    let loc = instr
        .split(['[', ']'])
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let final_val = apply(masks, val);
    mem.insert(loc, final_val);
}

pub fn part1(input: &str) -> u64 {
    let mut mem = HashMap::new();
    let mut masks = (0, 0);
    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();
        let instr = iter.next().unwrap();
        let val = iter.nth(1).unwrap();
        if instr == "mask" {
            masks = prepare_masks(val)
        } else {
            exec_mem_instr(&mut mem, masks, instr, val)
        }
    }
    mem.values().sum()
}

fn calc_all_locs(mut loc: Vec<char>) -> Vec<String> {
    if loc.is_empty() {
        return vec![String::new()];
    }
    let first = loc.remove(0);
    if first != 'X' {
        let mut rec_locs = calc_all_locs(loc);
        for l in rec_locs.iter_mut() {
            l.insert(0, first);
        }
        return rec_locs;
    }
    let mut new_locs = vec![];
    for rec_loc in calc_all_locs(loc) {
        new_locs.push(once('0').chain(rec_loc.chars()).collect());
        new_locs.push(once('1').chain(rec_loc.chars()).collect());
    }
    new_locs
}

fn apply_v2(mask: &str, loc: usize) -> Vec<String> {
    let n = mask.len();
    let loc = format!("{loc:0width$b}", width = n);
    let mut char_list = vec![];
    for (i, mask_bit) in mask.chars().enumerate() {
        if mask_bit == '1' {
            char_list.push('1')
        } else if mask_bit == 'X' {
            char_list.push('X')
        } else {
            char_list.push(loc.chars().nth(i).unwrap())
        }
    }
    calc_all_locs(char_list)
}

fn exec_mem_instr_v2(mem: &mut HashMap<usize, u64>, mask: &str, instr: &str, val: &str) {
    let loc = instr
        .split(['[', ']'])
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let val = val.parse().unwrap();
    let locations = apply_v2(mask, loc);
    for final_loc in locations {
        mem.insert(usize::from_str_radix(&final_loc, 2).unwrap(), val);
    }
}

pub fn part2(input: &str) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = "";
    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();
        let instr = iter.next().unwrap();
        let val = iter.nth(1).unwrap();
        if instr == "mask" {
            mask = val
        } else {
            exec_mem_instr_v2(&mut mem, mask, instr, val)
        }
    }
    mem.values().sum()
}
