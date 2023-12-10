use std::{collections::HashMap, iter::repeat};

use tinyvec::array_vec;

mod parse;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn zero() -> Self {
        Self::default()
    }

    fn merge_max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Info {
    #[default]
    Unknown,
    Tied,
    ClosesTo(char),
}

impl std::fmt::Debug for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "?"),
            Self::Tied => write!(f, "."),
            Self::ClosesTo(c) => write!(f, "{c}"),
        }
    }
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

fn info_from_neighbors(grid: &[Vec<Info>], y: usize, x: usize) -> Info {
    let mut neighbors = array_vec!([Info; 4]);

    if y > 0 {
        let new_neighbor = grid[y - 1][x];
        if !neighbors.contains(&new_neighbor) {
            neighbors.push(new_neighbor);
        }
    }
    if x > 0 {
        let new_neighbor = grid[y][x - 1];
        if !neighbors.contains(&new_neighbor) {
            neighbors.push(new_neighbor);
        }
    }
    if y + 1 < grid.len() {
        let new_neighbor = grid[y + 1][x];
        if !neighbors.contains(&new_neighbor) {
            neighbors.push(new_neighbor);
        }
    }
    if x + 1 < grid[0].len() {
        let new_neighbor = grid[y][x + 1];
        if !neighbors.contains(&new_neighbor) {
            neighbors.push(new_neighbor);
        }
    }

    neighbors.retain(|i| *i != Info::Unknown);

    match neighbors.len() {
        0 => Info::Unknown,
        1 => neighbors[0],
        _ => Info::Tied,
    }
}

// for debugging purposes
struct Displ<'a>(&'a [Vec<Info>]);

impl std::fmt::Display for Displ<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            for info in row {
                write!(f, "{info}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part1(mut input: &str) -> usize {
    let coordinates = parse::coordinates(&mut input).unwrap();

    let max = coordinates
        .iter()
        .fold(Coordinate::zero(), |acc, coord| acc.merge_max(coord));

    let mut grid: Vec<Vec<Info>> = vec![vec![Default::default(); max.x + 1]; max.y + 1];

    // fill with initial coordinates
    for (i, coord) in coordinates.into_iter().enumerate() {
        let chr = (i + 'a' as usize) as u8 as char;
        grid[coord.y][coord.x] = Info::ClosesTo(chr);
    }

    // println!("{}", Displ(&grid));

    let mut unknown_remaining = true;
    while unknown_remaining {
        unknown_remaining = false;
        let mut next_grid = grid.clone();

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == Info::Unknown {
                    let new_info = info_from_neighbors(&grid, y, x);
                    if new_info == Info::Unknown {
                        unknown_remaining = true;
                    } else {
                        next_grid[y][x] = new_info;
                    }
                }
            }
        }
        std::mem::swap(&mut grid, &mut next_grid);

        // println!("{}", Displ(&grid));
    }

    let mut area_size = HashMap::new();

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if let Info::ClosesTo(chr) = grid[y][x] {
                *area_size.entry(chr).or_default() += 1;
            }
        }
    }

    let top_row = repeat(0).zip(0..grid[0].len());
    let bot_row = repeat(grid.len() - 1).zip(0..grid[0].len());
    let fst_col = (0..grid.len()).zip(repeat(0));
    let lst_col = (0..grid.len()).zip(repeat(grid[0].len() - 1));

    let border = top_row.chain(bot_row).chain(fst_col).chain(lst_col);

    for (y, x) in border {
        if let Info::ClosesTo(chr) = grid[y][x] {
            area_size.remove(&chr);
        }
    }

    area_size.into_values().max().unwrap()
}

pub fn part2_impl(mut input: &str, max_dist: usize) -> usize {
    let coordinates = parse::coordinates(&mut input).unwrap();

    let max = coordinates
        .iter()
        .fold(Coordinate::zero(), |acc, coord| acc.merge_max(coord));

    let mut grid = vec![vec![0_isize; max.x + 1]; max.y + 1];

    // determine first distance (0,0) as starting point
    grid[0][0] = coordinates
        .iter()
        .map(|&Coordinate { x, y }| x + y)
        .sum::<usize>() as isize;

    // fill first column
    for y in 1..grid.len() {
        let x = 0;

        let coords_behind = coordinates.iter().filter(|coord| coord.y < y).count() as isize;
        let coords_in_front = coordinates.len() as isize - coords_behind;

        grid[y][x] = grid[y - 1][x] + coords_behind - coords_in_front;
    }

    // fill remaining columns
    for x in 1..grid[0].len() {
        let coords_behind = coordinates.iter().filter(|coord| coord.x < x).count() as isize;
        let coords_in_front = coordinates.len() as isize - coords_behind;
        let diff = coords_behind - coords_in_front;

        for row in grid.iter_mut() {
            row[x] = row[x - 1] + diff;
        }
    }
    // for row in grid.iter() {
    //     println!("{:?}", row);
    // }

    let max_dist = max_dist as isize;
    grid.into_iter()
        .flatten()
        .filter(|&dist| dist < max_dist)
        .count()
}

pub fn part2(input: &str) -> usize {
    part2_impl(input, 10_000)
}
