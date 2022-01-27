use std::vec::Vec;

struct Coordinates {
    x: i32,
    y: i32,
}

fn get_new_position(x: i32, y: i32, direction: char) -> Coordinates {
    match direction {
        '^' => Coordinates { x, y: y + 1 },
        '>' => Coordinates { x: x + 1, y },
        'v' => Coordinates { x, y: y - 1 },
        '<' => Coordinates { x: x - 1, y },
        _ => {
            dbg!("invalid input!");
            Coordinates { x: 0, y: 0 }
        }
    }
}

fn get_internal_coord(a: i32) -> usize {
    if a < 0 {
        ((a * -2) - 1) as usize
    } else {
        (a * 2) as usize
    }
}

fn part1(input: &str) {
    let (mut x, mut y) = (0, 0);
    let mut visited_houses: Vec<Vec<bool>> = vec![vec![]];
    visited_houses[0].push(true);
    let mut count = 1;
    for direction in input.chars() {
        let new_coord = get_new_position(x, y, direction);
        x = new_coord.x;
        y = new_coord.y;
        let x_int: usize = get_internal_coord(x);
        let y_int: usize = get_internal_coord(y);
        if x_int >= visited_houses.len() {
            visited_houses.resize(x_int + 1, Vec::new())
        };
        if y_int >= visited_houses[x_int].len() {
            visited_houses[x_int].resize(y_int + 1, false)
        };
        if !visited_houses[x_int][y_int] {
            visited_houses[x_int][y_int] = true;
            count += 1;
        }
    }
    println!("houses with presents: {}", count);
}

fn deliver_present(x: i32, y: i32, visited_houses: &mut Vec<Vec<bool>>) -> bool {
    let x_usize: usize = get_internal_coord(x);
    let y_usize: usize = get_internal_coord(y);
    if x_usize >= visited_houses.len() {
        visited_houses.resize(x_usize + 1, Vec::new())
    };
    if y_usize >= visited_houses[x_usize].len() {
        visited_houses[x_usize].resize(y_usize + 1, false)
    };
    if !visited_houses[x_usize][y_usize] {
        visited_houses[x_usize][y_usize] = true;
        true
    } else {
        false
    }
}

enum Santa {
    BioSanta,
    RoboSanta,
}

fn part2(input: &str) {
    let (mut x_bio, mut y_bio) = (0, 0);
    let (mut x_robo, mut y_robo) = (0, 0);
    let mut visited_houses: Vec<Vec<bool>> = vec![vec![]];
    visited_houses[0].push(true);
    let mut count = 1;
    let mut santa = Santa::BioSanta;
    for direction in input.chars() {
        match &mut santa {
            Santa::BioSanta => {
                let new_coord = get_new_position(x_bio, y_bio, direction);
                x_bio = new_coord.x;
                y_bio = new_coord.y;
                if deliver_present(x_bio, y_bio, &mut visited_houses) {
                    count += 1;
                }
                santa = Santa::RoboSanta;
            }
            Santa::RoboSanta => {
                let new_coord = get_new_position(x_robo, y_robo, direction);
                x_robo = new_coord.x;
                y_robo = new_coord.y;
                if deliver_present(x_robo, y_robo, &mut visited_houses) {
                    count += 1;
                }
                santa = Santa::BioSanta;
            }
        }
    }
    println!("houses with presents: {} (counter)", count);
}

fn part2_manual_iteration(input: &str) {
    let (mut x_bio, mut y_bio) = (0, 0);
    let (mut x_robo, mut y_robo) = (0, 0);
    let mut visited_houses: Vec<Vec<bool>> = vec![vec![]];
    visited_houses[0].push(true);
    let mut count = 1;
    let mut directions = input.chars();
    while let Some(bio_direction) = directions.next() {
        let mut new_coord = get_new_position(x_bio, y_bio, bio_direction);
        x_bio = new_coord.x;
        y_bio = new_coord.y;
        if deliver_present(x_bio, y_bio, &mut visited_houses) {
            count += 1;
        }
        let robo_direction = directions.next().unwrap();
        new_coord = get_new_position(x_robo, y_robo, robo_direction);
        x_robo = new_coord.x;
        y_robo = new_coord.y;
        if deliver_present(x_robo, y_robo, &mut visited_houses) {
            count += 1;
        }
    }
    println!("houses with presents: {} (manual iteration)", count);
}

fn part2_byte_idx(input: &str) {
    let (mut x_bio, mut y_bio) = (0, 0);
    let (mut x_robo, mut y_robo) = (0, 0);
    let mut visited_houses: Vec<Vec<bool>> = vec![vec![]];
    visited_houses[0].push(true);
    let mut count = 1;
    let directions = input.char_indices();
    for (byte_idx, direction) in directions {
        let new_coord = if byte_idx % 2 == 0 {
            let new_coord = get_new_position(x_bio, y_bio, direction);
            x_bio = new_coord.x;
            y_bio = new_coord.y;
            new_coord
        } else {
            let new_coord = get_new_position(x_robo, y_robo, direction);
            x_robo = new_coord.x;
            y_robo = new_coord.y;
            new_coord
        };
        if deliver_present(new_coord.x, new_coord.y, &mut visited_houses) {
            count += 1;
        }
    }
    println!("houses with presents: {} (byte index)", count);
}

fn main() {
    let input = include_str!("../input/input.txt");

    part1(input);

    println!("part 2:");
    part2(input);
    part2_manual_iteration(input);
    part2_byte_idx(input);
}
