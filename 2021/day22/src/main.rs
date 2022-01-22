use std::cmp::{max, min};
use std::ops::{Index, IndexMut, RangeInclusive};

use itertools::*;

type Line = (String, i32, i32, i32, i32, i32, i32);
fn parse(line: &str) -> Line {
    {
        let mut iter = line
            .split("..")
            .flat_map(|s| s.split(" x="))
            .flat_map(|s| s.split(",y="))
            .flat_map(|s| s.split(",z="));
        (
            iter.next().unwrap().parse::<String>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Range {
        let (start, end) = (min(start, end), max(start, end));
        Range { start, end }
    }

    fn beyond_init_area(&self) -> bool {
        self.start < -50 || self.end > 50
    }
}

impl IntoIterator for Range {
    type Item = i32;
    type IntoIter = RangeInclusive<i32>;
    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    state: bool,
    x: Range,
    y: Range,
    z: Range,
}

impl Cuboid {
    fn from_line(line: Line) -> Cuboid {
        let x = Range::new(line.1, line.2);
        let y = Range::new(line.3, line.4);
        let z = Range::new(line.5, line.6);
        Cuboid {
            state: &line.0 == "on",
            x,
            y,
            z,
        }
    }

    fn beyond_init_area(&self) -> bool {
        self.x.beyond_init_area() || self.y.beyond_init_area() || self.z.beyond_init_area()
    }

    fn init(self, reactor: &mut Reactor) {
        if self.beyond_init_area() {
            return;
        }
        self.exec(reactor);
    }

    fn exec(self, reactor: &mut Reactor) {
        for x in self.x {
            for y in self.y {
                for z in self.z {
                    reactor[(x, y, z)] = self.state;
                }
            }
        }
    }

    fn get_overlap(&self, other: &Self, overlap_state: bool) -> Option<Self> {
        let start_x = max(self.x.start, other.x.start);
        let start_y = max(self.y.start, other.y.start);
        let start_z = max(self.z.start, other.z.start);
        let end_x = min(self.x.end, other.x.end);
        let end_y = min(self.y.end, other.y.end);
        let end_z = min(self.z.end, other.z.end);
        if start_x > end_x || start_y > end_y || start_z > end_z {
            return None;
        }
        Some(Cuboid {
            state: overlap_state,
            x: Range::new(start_x, end_x),
            y: Range::new(start_y, end_y),
            z: Range::new(start_z, end_z),
        })
    }

    fn size(&self) -> usize {
        let x_range = (self.x.end - self.x.start + 1) as usize;
        let y_range = (self.y.end - self.y.start + 1) as usize;
        let z_range = (self.z.end - self.z.start + 1) as usize;
        x_range * y_range * z_range
    }
}

struct Reactor {
    cells: Vec<Vec<Vec<bool>>>,
}

type ReactorIdx = (i32, i32, i32);

impl Reactor {
    fn new() -> Reactor {
        Reactor {
            cells: vec![vec![vec![false; 101]; 101]; 101],
        }
    }

    fn convert_index_part(i: i32) -> usize {
        if i < 0 {
            ((-i * 2) - 1) as usize
        } else {
            (i * 2) as usize
        }
    }

    fn convert_index((x, y, z): ReactorIdx) -> (usize, usize, usize) {
        (
            Self::convert_index_part(x),
            Self::convert_index_part(y),
            Self::convert_index_part(z),
        )
    }

    fn count_active(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|plane| {
                plane
                    .iter()
                    .flat_map(|line| line.iter().filter(|&&cell| cell))
            })
            .count()
    }
}

impl Index<ReactorIdx> for Reactor {
    type Output = bool;

    fn index(&self, idx: ReactorIdx) -> &<Self as Index<ReactorIdx>>::Output {
        let (x, y, z) = Reactor::convert_index(idx);
        if x < self.cells.len() && y < self.cells[x].len() && z < self.cells[x][y].len() {
            &self.cells[x][y][z]
        } else {
            &false
        }
    }
}

impl IndexMut<ReactorIdx> for Reactor {
    fn index_mut(&mut self, idx: ReactorIdx) -> &mut <Self as Index<ReactorIdx>>::Output {
        let (x, y, z) = Reactor::convert_index(idx);
        if x >= self.cells.len() {
            self.cells.resize(x * 2, vec![vec![false; 101]; 101]);
        }
        if y >= self.cells[x].len() {
            self.cells[x].resize(y * 2, vec![false; 101]);
        }
        if z >= self.cells[x][y].len() {
            self.cells[x][y].resize(z * 2, false);
        }
        &mut self.cells[x][y][z]
    }
}

fn part1(cuboids: Vec<Cuboid>) {
    let mut reactor = Reactor::new();

    for cuboid in cuboids {
        cuboid.init(&mut reactor);
    }

    println!("{:?}", reactor.count_active())
}

#[derive(Debug, Clone)]
struct Reactor2 {
    executed: Vec<Cuboid>,
}

impl Reactor2 {
    fn new() -> Reactor2 {
        Reactor2 {
            executed: Vec::new(),
        }
    }

    fn execute(&mut self, cuboid: Cuboid) {
        let mut newly_executed = Vec::new();
        for other in self.executed.iter() {
            if let Some(overlap) = cuboid.get_overlap(other, !other.state) {
                newly_executed.push(overlap);
            }
        }
        self.executed.append(&mut newly_executed);
        if cuboid.state {
            self.executed.push(cuboid);
        }
    }

    fn count_active(&self) -> usize {
        let mut sum = 0;
        for executed in self.executed.iter() {
            if executed.state {
                sum += executed.size();
            } else {
                sum -= executed.size();
            }
        }
        sum
    }
}

fn part2(cuboids: Vec<Cuboid>) {
    let mut reactor = Reactor2::new();

    for cuboid in cuboids {
        reactor.execute(cuboid)
    }

    println!("{:?}", reactor.count_active())
}

fn main() {
    let input = include_str!("../input/input.txt")
        .lines()
        .map(parse)
        .map(Cuboid::from_line)
        .collect_vec();

    part1(input.clone()); // 543306

    part2(input);
}
