use pathfinding::prelude::*;
use utils::Itertools;

fn elevation(b: u8) -> u8 {
    match b {
        b'S' => b'a',
        b'E' => b'z',
        _ => b,
    }
}

type Idx = (usize, usize);

fn can_walk_to(grid: &[Vec<u8>], center: Idx, other: Idx) -> Option<Idx> {
    (elevation(grid[center.0][center.1]) + 1 >= elevation(grid[other.0][other.1])).then_some(other)
}

fn top(grid: &[Vec<u8>], idx: (usize, usize)) -> Option<Idx> {
    if idx.0 == 0 {
        return None;
    }
    can_walk_to(grid, idx, (idx.0 - 1, idx.1))
}

fn bottom(grid: &[Vec<u8>], idx: (usize, usize)) -> Option<Idx> {
    if idx.0 + 1 == grid.len() {
        return None;
    }
    can_walk_to(grid, idx, (idx.0 + 1, idx.1))
}

fn left(grid: &[Vec<u8>], idx: (usize, usize)) -> Option<Idx> {
    if idx.1 == 0 {
        return None;
    }
    can_walk_to(grid, idx, (idx.0, idx.1 - 1))
}

fn right(grid: &[Vec<u8>], idx: (usize, usize)) -> Option<Idx> {
    if idx.1 + 1 == grid[0].len() {
        return None;
    }
    can_walk_to(grid, idx, (idx.0, idx.1 + 1))
}

pub fn part1(input: &str) -> usize {
    let grid = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();

    let mut all_squares = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, b)| (i, j, b)));
    let start = all_squares
        .clone()
        .find_map(|(i, j, b)| (*b == b'S').then_some((i, j)))
        .unwrap();
    let goal = all_squares
        .find_map(|(i, j, b)| (*b == b'E').then_some((i, j)))
        .unwrap();

    let path = dijkstra(
        &start,
        move |&idx| {
            let t = top(&grid, idx);
            let b = bottom(&grid, idx);
            let l = left(&grid, idx);
            let r = right(&grid, idx);
            [t, b, l, r].into_iter().flatten().map(|idx| (idx, 1))
        },
        |&idx| idx == goal,
    );

    path.unwrap().1
}

pub fn part2(input: &str) -> usize {
    let grid = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();

    let mut all_squares = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, b)| (i, j, b)));
    let start_options = all_squares
        .clone()
        .filter_map(|(i, j, b)| (elevation(*b) == b'a').then_some((i, j)));
    let goal = all_squares
        .find_map(|(i, j, b)| (*b == b'E').then_some((i, j)))
        .unwrap();

    start_options
        .flat_map(|start| {
            dijkstra(
                &start,
                |&idx| {
                    let t = top(&grid, idx);
                    let b = bottom(&grid, idx);
                    let l = left(&grid, idx);
                    let r = right(&grid, idx);
                    [t, b, l, r].into_iter().flatten().map(|idx| (idx, 1))
                },
                |&idx| idx == goal,
            )
            .map(|p| p.1)
        })
        .min()
        .unwrap()
}
