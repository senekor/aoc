use itertools::*;

use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct Node {
    i: usize,
    j: usize,
    risk_level: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.i == other.i && self.j == other.j && self.risk_level == other.risk_level;
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.risk_level.cmp(&other.risk_level));
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.risk_level.cmp(&other.risk_level);
    }
}

fn dykstra(grid: &mut Vec<Vec<(usize, bool)>>) -> usize {
    let mut options = vec![Node {
        i: 0,
        j: 0,
        risk_level: 0,
    }];
    grid[0][0].1 = true;
    let (mut i, mut j, mut risk_level);
    loop {
        let opt = options.swap_remove(0);
        i = opt.i;
        j = opt.j;
        risk_level = opt.risk_level;
        if i == grid.len() - 1 && j == grid[0].len() - 1 {
            return risk_level;
        }
        if i != 0 && !grid[i - 1][j].1 {
            options.push(Node {
                i: i - 1,
                j,
                risk_level: risk_level + grid[i - 1][j].0,
            });
            grid[i - 1][j].1 = true;
        }
        if j != 0 && !grid[i][j - 1].1 {
            options.push(Node {
                i: i,
                j: j - 1,
                risk_level: risk_level + grid[i][j - 1].0,
            });
            grid[i][j - 1].1 = true;
        }
        if i != grid.len() - 1 && !grid[i + 1][j].1 {
            options.push(Node {
                i: i + 1,
                j,
                risk_level: risk_level + grid[i + 1][j].0,
            });
            grid[i + 1][j].1 = true;
        }
        if j != grid[0].len() - 1 && !grid[i][j + 1].1 {
            options.push(Node {
                i: i,
                j: j + 1,
                risk_level: risk_level + grid[i][j + 1].0,
            });
            grid[i][j + 1].1 = true;
        }
        options.sort();
    }
}

fn part1(mut grid: Vec<Vec<(usize, bool)>>) {
    let risk_level = dykstra(&mut grid);
    println!("{:?}", risk_level)
}

fn part2(mut grid: Vec<Vec<(usize, bool)>>) {
    let num_new_cols = grid[0].len() * 4;
    for i in 0..grid.len() {
        for j in 0..num_new_cols {
            let next = (grid[i][j].0 % 9 + 1, false);
            grid[i].push(next);
        }
    }

    let num_new_lines = grid.len() * 4;
    for i in 0..num_new_lines {
        grid.push(
            grid[i]
                .iter()
                .map(|(r, _)| (*r % 9 + 1, false))
                .collect_vec(),
        );
    }

    let risk_level = dykstra(&mut grid);
    println!("{:?}", risk_level)
}

fn main() {
    let grid = include_str!("../input/input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_string().parse::<usize>().unwrap(), false))
                .collect_vec()
        })
        .collect_vec();

    part1(grid.clone());

    part2(grid);
}
