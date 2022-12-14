use std::cmp;
use std::fs;
use std::string::String;
use std::time::Instant;

fn count_both(input: &str) -> (usize, usize, usize) {
    let mut left_count = 0;
    let mut right_count = 0;
    for character in input.chars() {
        if character == '(' {
            left_count += 1
        } else {
            right_count += 1
        }
    }
    (left_count, right_count, left_count - right_count)
}

fn count_once(input: &str) -> (usize, usize, usize) {
    let mut left_count = 0;
    for character in input.chars() {
        if character == '(' {
            left_count += 1
        }
    }
    let right_count = input.len() - left_count;
    (left_count, right_count, left_count - right_count)
}

const MASK: u8 = 0b0000_0001;

fn bitwise(input: &str) -> (usize, usize, usize) {
    let mut right_count: usize = 0;
    for byte in input.bytes() {
        right_count += (byte & MASK) as usize;
    }
    let left_count = input.len() - right_count;
    (left_count, right_count, left_count - right_count)
}

pub fn part1(input: &str) -> usize {
    let before_count_both = Instant::now();
    let (up, down, floor) = count_both(input);
    let count_both_duration = before_count_both.elapsed();

    println!("count both:");
    println!("left: {}, right: {}, floor: {}", up, down, floor);
    println!("time elapsed in ns: {}", count_both_duration.as_micros());

    let before_count_once = Instant::now();
    let (up_once, down_once, floor_once) = count_once(input);
    let count_once_duration = before_count_once.elapsed();

    println!("count once:");
    println!(
        "left: {}, right: {}, floor: {}",
        up_once, down_once, floor_once
    );
    println!("time elapsed in ns: {}", count_once_duration.as_micros());

    let before_bitwise = Instant::now();
    let (up_bw, down_bw, floor_bw) = bitwise(input);
    let bitwise_duration = before_bitwise.elapsed();

    println!("bitwise:");
    println!("left: {}, right: {}, floor: {}", up_bw, down_bw, floor_bw);
    println!("time elapsed in ns: {}", bitwise_duration.as_micros());

    floor
}

pub fn part2(input: &str) -> usize {
    let mut floor = 0;
    let bytes = input.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == 0b0010_1000 {
            floor += 1
        } else {
            floor -= 1
        }
        if floor < 0 {
            return i + 1;
        }
    }
    panic!()
}

fn get_bucket_idx(bucket_size: i32, floor: i32) -> usize {
    let half_bucket_size = bucket_size / 2;
    if floor < half_bucket_size && floor > -half_bucket_size {
        50
    } else {
        let abs_floor = floor.abs();
        let offset_floor = abs_floor - half_bucket_size;
        let offset_index = offset_floor / bucket_size + 1;
        if floor > 0 {
            50 + offset_index as usize
        } else {
            50 - offset_index as usize
        }
    }
}

pub fn graph(input: &str) {
    let mut current_floor = 0;
    let mut max_floor = 0;
    let mut min_floor = 0;
    let mut floors: Vec<i32> = Vec::with_capacity(input.len());
    let bytes = input.as_bytes();
    for &b in bytes.iter() {
        if b == 0b0010_1000 {
            current_floor += 1;
            if current_floor > max_floor {
                max_floor = current_floor
            }
        } else {
            current_floor -= 1;
            if current_floor < min_floor {
                min_floor = current_floor
            }
        }
        floors.push(current_floor)
    }
    let floor_range = cmp::max(max_floor, -min_floor) * 2;
    let bucket_size = floor_range / 100 + 1;

    let mut output = String::new();
    for floor in floors {
        let bucket_idx = get_bucket_idx(bucket_size, floor);
        let empty_space = " ".repeat(50);
        let mut line = format!("{}.{}\n", empty_space, empty_space);
        line.insert(bucket_idx, 'x');
        output += line.as_str();
    }
    match fs::write("output/output.txt", output) {
        Ok(_) => {
            println!("ok")
        }
        Err(x) => {
            println!("err: {}", x)
        }
    }
}
