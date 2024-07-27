// basic idea:
// Find all _lines_ on which at least two asteroids lie.
// The algorithm for this can be pretty stupid, n is quite small.
// Next, iterate over all lines and assign penalties to each asteroid
// based on how many others are obstructed on that line.
// The asteroid with the fewest penalties win.
//
// The representation of a line should make it easy to calculate
// if a given asteroid lies on the line or not.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub fn part1(input: &str) -> usize {
    let asteroids = input
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
        .collect::<Vec<_>>();

    let mut max = 0;
    for monitor in iter_asteroids(&asteroids) {
        let mut count = 0;
        for target in iter_asteroids(&asteroids) {
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
            max = count
        }
    }

    max
}

pub fn part2(input: &str) -> usize {
    0
}
