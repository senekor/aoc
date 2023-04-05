use nalgebra as na;

type Point = na::Point2<i32>;
type Vector = na::Vector2<i32>;
type InternalPoint = na::Point2<usize>;

fn get_new_position(point: Point, direction: char) -> Point {
    match direction {
        '^' => point + Vector::new(0, 1),
        '>' => point + Vector::new(1, 0),
        'v' => point + Vector::new(0, -1),
        '<' => point + Vector::new(-1, 0),
        d => panic!("invalid direction: {d}"),
    }
}

fn get_internal_coord(a: i32) -> usize {
    if a < 0 {
        ((a * -2) - 1) as usize
    } else {
        (a * 2) as usize
    }
}

fn get_internal_point(point: Point) -> InternalPoint {
    InternalPoint::new(get_internal_coord(point.x), get_internal_coord(point.y))
}

pub fn part1(input: &str) -> usize {
    let mut position = Point::new(0, 0);
    let mut visited_houses: Vec<Vec<bool>> = vec![vec![]];
    visited_houses[0].push(true);
    let mut count = 1;
    for direction in input.chars() {
        position = get_new_position(position, direction);
        let internal_pos = get_internal_point(position);
        let (x_int, y_int) = (internal_pos.x, internal_pos.y);
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
    count
}

fn deliver_present(position: Point, visited_houses: &mut Vec<Vec<bool>>) -> bool {
    let internal_pos = get_internal_point(position);
    let (x_int, y_int) = (internal_pos.x, internal_pos.y);
    if x_int >= visited_houses.len() {
        visited_houses.resize(x_int + 1, Vec::new())
    };
    if y_int >= visited_houses[x_int].len() {
        visited_houses[x_int].resize(y_int + 1, false)
    };
    if !visited_houses[x_int][y_int] {
        visited_houses[x_int][y_int] = true;
        true
    } else {
        false
    }
}

enum Santa {
    BioSanta,
    RoboSanta,
}

pub fn part2(input: &str) -> usize {
    let mut pos_bio = Point::new(0, 0);
    let mut pos_robo = Point::new(0, 0);
    let mut visited_houses: Vec<Vec<bool>> = vec![vec![]];
    visited_houses[0].push(true);
    let mut count = 1;
    let mut santa = Santa::BioSanta;
    for direction in input.chars() {
        match &mut santa {
            Santa::BioSanta => {
                pos_bio = get_new_position(pos_bio, direction);
                if deliver_present(pos_bio, &mut visited_houses) {
                    count += 1;
                }
                santa = Santa::RoboSanta;
            }
            Santa::RoboSanta => {
                pos_robo = get_new_position(pos_robo, direction);
                if deliver_present(pos_robo, &mut visited_houses) {
                    count += 1;
                }
                santa = Santa::BioSanta;
            }
        }
    }
    count
}

// fn part2_manual_iteration(input: &str) {
//     let (mut x_bio, mut y_bio) = (0, 0);
//     let (mut x_robo, mut y_robo) = (0, 0);
//     let mut visited_houses: Vec<Vec<bool>> = vec![vec![]];
//     visited_houses[0].push(true);
//     let mut count = 1;
//     let mut directions = input.chars();
//     while let Some(bio_direction) = directions.next() {
//         let mut new_coord = get_new_position(x_bio, y_bio, bio_direction);
//         x_bio = new_coord.x;
//         y_bio = new_coord.y;
//         if deliver_present(x_bio, y_bio, &mut visited_houses) {
//             count += 1;
//         }
//         let robo_direction = directions.next().unwrap();
//         new_coord = get_new_position(x_robo, y_robo, robo_direction);
//         x_robo = new_coord.x;
//         y_robo = new_coord.y;
//         if deliver_present(x_robo, y_robo, &mut visited_houses) {
//             count += 1;
//         }
//     }
//     println!("houses with presents: {} (manual iteration)", count);
// }

// fn part2_byte_idx(input: &str) {
//     let (mut x_bio, mut y_bio) = (0, 0);
//     let (mut x_robo, mut y_robo) = (0, 0);
//     let mut visited_houses: Vec<Vec<bool>> = vec![vec![]];
//     visited_houses[0].push(true);
//     let mut count = 1;
//     let directions = input.char_indices();
//     for (byte_idx, direction) in directions {
//         let new_coord = if byte_idx % 2 == 0 {
//             let new_coord = get_new_position(x_bio, y_bio, direction);
//             x_bio = new_coord.x;
//             y_bio = new_coord.y;
//             new_coord
//         } else {
//             let new_coord = get_new_position(x_robo, y_robo, direction);
//             x_robo = new_coord.x;
//             y_robo = new_coord.y;
//             new_coord
//         };
//         if deliver_present(new_coord.x, new_coord.y, &mut visited_houses) {
//             count += 1;
//         }
//     }
//     println!("houses with presents: {} (byte index)", count);
// }
