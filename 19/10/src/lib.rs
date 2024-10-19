use std::f64::consts::PI;

// Coordinate system:
//
// +---> [x]
// |
// |
// v [y]
//
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Point {
    x: usize,
    y: usize,
}

fn iter_asteroids(asteroids: &[Vec<bool>]) -> impl Iterator<Item = Point> + '_ {
    asteroids.iter().enumerate().flat_map(|(y, row)| {
        row.iter()
            .copied()
            .enumerate()
            .flat_map(move |(x, is_asteroid)| is_asteroid.then_some(Point { x, y }))
    })
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("asteroid or no asteroid, that is the question."),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn find_monitor(asteroids: &[Vec<bool>]) -> (Point, usize) {
    let mut res = Point::default();
    let mut max = 0;

    for monitor in iter_asteroids(asteroids) {
        let mut count = 0;
        for target in iter_asteroids(asteroids) {
            if monitor == target {
                continue;
            }
            // determine location on line between monitor and target
            let dx = monitor.x.abs_diff(target.x);
            let dy = monitor.y.abs_diff(target.y);
            let gcd = num::integer::gcd(dx, dy);
            let (step_x, step_y) = {
                let mut step_x = (dx / gcd) as isize;
                let mut step_y = (dy / gcd) as isize;
                if monitor.x > target.x {
                    step_x = -step_x;
                }
                if monitor.y > target.y {
                    step_y = -step_y;
                }
                (step_x, step_y)
            };
            let mut between = monitor;
            loop {
                between.x = between.x.wrapping_add_signed(step_x);
                between.y = between.y.wrapping_add_signed(step_y);
                if between == target {
                    count += 1;
                    break;
                }
                if asteroids[between.y][between.x] {
                    break;
                }
            }
        }
        if count > max {
            max = count;
            res = monitor;
        }
    }

    (res, max)
}

pub fn part1(input: &str) -> usize {
    let asteroids = parse(input);
    find_monitor(&asteroids).1
}

pub fn part2(input: &str) -> usize {
    let asteroids = parse(input);
    let monitor = find_monitor(&asteroids).0;

    // invariant: the target queue is a sorted list of angles and the asteroids
    // at that angle. the asteroids are another list, sorted by distance.
    //
    // [
    //   (1, [1, 2]),
    //   (2, [3]),
    //   (4, [1, 2, 4]),
    // ]
    let mut target_queue: Vec<(f64, Vec<(usize, Point)>)> = Vec::new();

    for target in iter_asteroids(&asteroids) {
        if monitor == target {
            continue;
        }
        let dx = monitor.x.abs_diff(target.x);
        let dy = monitor.y.abs_diff(target.y);
        let gcd = num::integer::gcd(dx, dy);
        let (dx, dy) = (dx / gcd, dy / gcd);

        let angle = if target.x < monitor.x {
            if target.y < monitor.y {
                // fourth quadrant
                2.0 * PI - f64::atan(dx as f64 / dy as f64)
            } else {
                // third quadrant
                PI + f64::atan(dx as f64 / dy as f64)
            }
        } else if target.y > monitor.y {
            // second quadrant
            PI - f64::atan(dx as f64 / dy as f64)
        } else {
            f64::atan(dx as f64 / dy as f64)
        };

        match target_queue.binary_search_by(|x| x.0.partial_cmp(&angle).unwrap()) {
            Ok(idx) => {
                let asteroids_on_line = &mut target_queue[idx].1;
                let to_insert = (gcd, target);
                let idx = asteroids_on_line
                    .binary_search(&to_insert)
                    .expect_err("dupliacate asteroid?");
                asteroids_on_line.insert(idx, to_insert);
            }
            Err(idx) => target_queue.insert(idx, (angle, vec![(gcd, target)])),
        }
    }

    let mut idx = 0;
    // remove first 199 asteroids to be lasered
    for _ in 0..199 {
        if target_queue[idx].1.len() == 1 {
            target_queue.remove(idx);
            continue;
        }
        target_queue[idx].1.remove(0);
        idx = (idx + 1) % target_queue.len();
    }
    let res = target_queue[idx].1[0].1;

    res.x * 100 + res.y
}
