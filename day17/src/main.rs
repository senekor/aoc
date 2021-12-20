use itertools::*;
use std::cmp::{max, min};

use crate::projectile::Projectile;

#[derive(Debug)]
struct Target {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl Target {
    fn new(s: &str) -> Target {
        let nums = s
            .trim_start_matches("target area: x=")
            .split(", y=")
            .flat_map(|x| x.split(".."))
            .map(|s| s.parse::<i32>().unwrap())
            .collect_vec();
        Target {
            left: nums[0],
            right: nums[1],
            top: max(nums[2], nums[3]),
            bottom: min(nums[2], nums[3]),
        }
    }
}

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

mod projectile {
    use super::Point;
    use std::cmp::max;

    #[derive(Debug)]
    pub struct Projectile {
        x_vel: i32,
        y_vel: i32,
        coordinates: Point,
    }

    impl Projectile {
        pub fn new(x_vel: i32, y_vel: i32) -> Projectile {
            Projectile {
                x_vel,
                y_vel,
                coordinates: Point { x: 0, y: 0 },
            }
        }
    }

    impl Iterator for Projectile {
        type Item = Point;
        fn next(&mut self) -> Option<Self::Item> {
            self.coordinates.x += self.x_vel;
            self.coordinates.y += self.y_vel;
            self.x_vel = max(self.x_vel - 1, 0);
            self.y_vel -= 1;
            Some(Point {
                x: self.coordinates.x,
                y: self.coordinates.y,
            })
        }
    }
}

enum PtCmpResult {
    Miss,
    Hit,
    TooFar,
}
use PtCmpResult::*;

fn compare_point_to_target(point: &Point, target: &Target) -> PtCmpResult {
    match () {
        _ if point.x > target.right => TooFar,
        _ if point.y < target.bottom => TooFar,
        _ if point.x < target.left => Miss,
        _ if point.y > target.top => Miss,
        _ => Hit,
    }
}

fn projectile_hits_target(projectile: Projectile, target: &Target) -> bool {
    for point in projectile {
        match compare_point_to_target(&point, target) {
            Miss => {}
            Hit => return true,
            TooFar => return false,
        }
    }
    return false;
}

fn main() {
    let target = Target::new(include_str!("../input/input.txt"));

    // part 1
    println!("{:?}", (0..-target.bottom).sum::<i32>());

    let mut sum = 0;

    for x_vel in 0..=target.right {
        for y_vel in target.bottom..=-target.bottom {
            let projectile = Projectile::new(x_vel, y_vel);
            if projectile_hits_target(projectile, &target) {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}
