use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
struct Grid(Vec<Vec<bool>>, bool);

impl Grid {
    fn new(size: usize, four_corners: bool) -> Self {
        let internal_size = size + 2;
        let mut row = Vec::with_capacity(internal_size);
        row.resize(internal_size, false);
        let mut grid = Grid(Vec::with_capacity(internal_size), four_corners);
        grid.0.resize(internal_size, row);
        if four_corners {
            grid.turn_corners_on();
        }
        grid
    }
    fn count_neighbors(&self, i: usize, j: usize) -> usize {
        [
            self.0[i - 1][j - 1],
            self.0[i - 1][j],
            self.0[i - 1][j + 1],
            self.0[i][j - 1],
            self.0[i][j + 1],
            self.0[i + 1][j - 1],
            self.0[i + 1][j],
            self.0[i + 1][j + 1],
        ]
        .iter()
        .filter(|n| **n)
        .count()
    }
    fn animate(&mut self) {
        let mut next = Grid::new(self.0.len() - 2, false).0;
        for (i, row) in next.iter_mut().enumerate().take(self.0.len() - 1).skip(1) {
            for (j, cell) in row.iter_mut().enumerate().take(self.0.len() - 1).skip(1) {
                let n = self.count_neighbors(i, j);
                *cell = n == 3 || (self.0[i][j] && n == 2);
            }
        }
        self.0 = next;
        if self.1 {
            self.turn_corners_on();
        }
    }
    fn count_lights(&self) -> usize {
        self.0
            .iter()
            .flat_map(|row| row.iter())
            .filter(|b| **b)
            .count()
    }
    fn turn_corners_on(&mut self) {
        let size = self.0.len() - 2;
        self.0[1][1] = true;
        self.0[1][size] = true;
        self.0[size][1] = true;
        self.0[size][size] = true;
    }
    fn toggle_four_corners(&mut self) {
        self.1 = true;
        self.turn_corners_on();
    }
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = s.lines().count();
        let mut grid = Grid::new(size, false);
        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                grid.0[i + 1][j + 1] = matches!(c, '#')
            }
        }
        Ok(grid)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 1..self.0.len() - 1 {
            for j in 1..self.0.len() - 1 {
                if self.0[i][j] {
                    s = format!("{}{}", s, '#')
                } else {
                    s = format!("{}{}", s, '.')
                }
            }
            s = format!("{}\n", s)
        }
        write!(f, "{}", s)
    }
}

fn part1(input: &str) {
    let mut grid = Grid::from_str(input).unwrap();
    for _ in 0..100 {
        grid.animate();
    }
    println!("number of lights after 100 steps: {}", grid.count_lights());
}

fn part2(input: &str) {
    let mut grid = Grid::from_str(input).unwrap();
    grid.toggle_four_corners();
    for _ in 0..100 {
        grid.animate();
    }
    println!("number of lights after 100 steps: {}", grid.count_lights());
}

fn main() {
    let input = include_str!("../input/input.txt");
    // let input = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
    part1(input);
    part2(input);
}
