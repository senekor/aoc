#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Obstacle,
    Visited,
}

use Cell::*;

pub fn part1(input: &str) -> usize {
    let mut grid = Vec::new();
    let mut guard = (0, 0);
    let mut dir = (0, -1);

    for (y, line) in input.lines().enumerate() {
        grid.push(vec![Empty; line.len()]);
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => grid[y][x] = Obstacle,
                '^' => {
                    grid[y][x] = Visited;
                    guard = (x, y);
                }
                '.' => {}
                _ => panic!("unknown char: '{c}'"),
            }
        }
    }

    let get_next = |grid: &[Vec<Cell>], guard: (usize, usize), dir: (isize, isize)| {
        let x = guard.0.checked_add_signed(dir.0)?;
        let y = guard.1.checked_add_signed(dir.1)?;
        if x == grid[0].len() || y == grid.len() {
            return None;
        }
        Some((x, y))
    };

    while let Some(mut next) = get_next(&grid, guard, dir) {
        if grid[next.1][next.0] == Obstacle {
            // turn
            dir = match dir {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!("unknown direction {dir:?}"),
            };
            if let Some(n) = get_next(&grid, guard, dir) {
                next = n
            } else {
                break;
            }
        }
        guard = next;

        grid[guard.1][guard.0] = Visited;
    }

    grid.into_iter()
        .flatten()
        .filter(|cell| *cell == Visited)
        .count()
}

pub fn part2(input: &str) -> usize {
    0
}
