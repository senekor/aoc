#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Obstacle,
    Visited([bool; 4]), // one for each direction
}

use Cell::*;
use Direction::*;

fn parse(input: &str) -> (Vec<Vec<Cell>>, (usize, usize)) {
    let mut grid = Vec::new();
    let mut guard = (0, 0);

    for (y, line) in input.lines().enumerate() {
        grid.push(vec![Empty; line.len()]);
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => grid[y][x] = Obstacle,
                '^' => {
                    let mut visited = [false; 4];
                    visited[Up as usize] = true;
                    grid[y][x] = Visited(visited);
                    guard = (x, y);
                }
                '.' => {}
                _ => panic!("unknown char: '{c}'"),
            }
        }
    }

    (grid, guard)
}

pub fn part1(input: &str) -> usize {
    let (grid, guard) = parse(input);
    path_length(grid, guard, Up).unwrap()
}

fn get_next(grid: &[Vec<Cell>], guard: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
    let (dx, dy) = match dir {
        Up => (0, -1),
        Down => (0, 1),
        Left => (-1, 0),
        Right => (1, 0),
    };
    let x = guard.0.checked_add_signed(dx)?;
    let y = guard.1.checked_add_signed(dy)?;
    if x == grid[0].len() || y == grid.len() {
        return None;
    }
    Some((x, y))
}

/// Get the length of the guard's path, given a grid and starting position.
/// Returns `None` if the guard gets stuck in a loop.
fn path_length(
    mut grid: Vec<Vec<Cell>>,
    mut guard: (usize, usize),
    mut dir: Direction,
) -> Option<usize> {
    while let Some(mut next) = get_next(&grid, guard, dir) {
        while grid[next.1][next.0] == Obstacle {
            // turn
            dir = match dir {
                Up => Right,
                Down => Left,
                Left => Up,
                Right => Down,
            };
            if let Some(n) = get_next(&grid, guard, dir) {
                next = n
            } else {
                break;
            }
        }
        guard = next;

        match grid[guard.1][guard.0] {
            Empty => {
                let mut visited = [false; 4];
                visited[dir as usize] = true;
                grid[guard.1][guard.0] = Visited(visited)
            }
            Obstacle => unreachable!("tried to walk into obstacle"),
            Visited(ref mut visited) => {
                if visited[dir as usize] {
                    // found loop
                    return None;
                }
                visited[dir as usize] = true;
            }
        }
    }

    Some(
        grid.into_iter()
            .flatten()
            .filter(|cell| matches!(cell, Visited(_)))
            .count(),
    )
}

pub fn part2(input: &str) -> usize {
    let (grid, guard) = parse(input);

    // inefficient brute force...
    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !matches!(grid[y][x], Empty) {
                continue;
            }
            let mut grid = grid.clone();
            grid[y][x] = Obstacle;

            if path_length(grid, guard, Up).is_none() {
                // loop found
                count += 1;
            }
        }
    }

    count
}
