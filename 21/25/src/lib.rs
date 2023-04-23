use std::{
    fmt::{Display, Formatter},
    ops::{Index, IndexMut},
    str::FromStr,
};

use utils::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Seafloor {
    grid: Vec<Vec<char>>,
}

impl FromStr for Seafloor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Seafloor {
            grid: s
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        })
    }
}

impl Index<(usize, usize)> for Seafloor {
    type Output = char;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let i = index.0 % self.grid.len();
        let j = index.1 % self.grid[0].len();
        &self.grid[i][j]
    }
}

impl IndexMut<(usize, usize)> for Seafloor {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let i = index.0 % self.grid.len();
        let j = index.1 % self.grid[0].len();
        &mut self.grid[i][j]
    }
}

impl Display for Seafloor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(
            self.grid
                .iter()
                .flat_map(|row| row.iter().chain([&'\n'].into_iter()))
                .collect::<String>()
                .as_str(),
        )
    }
}

impl Seafloor {
    fn new(num_rows: usize, num_cols: usize) -> Seafloor {
        Seafloor {
            grid: vec![vec!['.'; num_cols]; num_rows],
        }
    }

    fn num_rows(&self) -> usize {
        self.grid.len()
    }

    fn num_cols(&self) -> usize {
        self.grid[0].len()
    }
}

pub fn part1(input: &str) -> i32 {
    let mut seafloor: Seafloor = input.parse().unwrap();
    for k in 1.. {
        let mut nothing_moved = true;

        let mut next_state = Seafloor::new(seafloor.num_rows(), seafloor.num_cols());

        // eastward first
        for i in 0..seafloor.num_rows() {
            for j in 0..seafloor.num_cols() {
                if seafloor[(i, j)] == '>' {
                    if seafloor[(i, j + 1)] == '.' {
                        next_state[(i, j + 1)] = '>';
                        nothing_moved = false;
                    } else {
                        next_state[(i, j)] = '>';
                    }
                } else if seafloor[(i, j)] == 'v' {
                    next_state[(i, j)] = 'v';
                }
            }
        }

        seafloor = next_state;
        next_state = Seafloor::new(seafloor.num_rows(), seafloor.num_cols());

        // soutward second
        for i in 0..seafloor.num_rows() {
            for j in 0..seafloor.num_cols() {
                if seafloor[(i, j)] == 'v' {
                    if seafloor[(i + 1, j)] == '.' {
                        next_state[(i + 1, j)] = 'v';
                        nothing_moved = false;
                    } else {
                        next_state[(i, j)] = 'v';
                    }
                } else if seafloor[(i, j)] == '>' {
                    next_state[(i, j)] = '>';
                }
            }
        }

        if nothing_moved {
            return k;
        }

        seafloor = next_state;
    }
    panic!()
}

pub fn part2(_input: &str) -> &'static str {
    "No part 2 for this puzzle :-)"
}
