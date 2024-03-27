use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use nalgebra::Point2;
use itertools::Itertools;

mod parse;

type Point = Point2<usize>;
type Path = Vec<Point>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Space {
    #[default]
    Air,
    Rock,
    Sand,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::Air => write!(f, "."),
            Space::Rock => write!(f, "#"),
            Space::Sand => write!(f, "+"),
        }
    }
}

const ENTRY_POINT: Point = Point::new(500, 0);

struct Cave {
    spaces: Vec<Space>,
    min_left: usize,
    width: usize,
    depth: usize,
}

impl Index<Point> for Cave {
    type Output = Space;

    fn index(&self, index: Point) -> &Self::Output {
        let index = self.point_to_internal_index(index);
        &self.spaces[index]
    }
}

impl IndexMut<Point> for Cave {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let index = self.point_to_internal_index(index);
        &mut self.spaces[index]
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.spaces.iter().chunks(self.width).into_iter() {
            for space in line {
                write!(f, "{space}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct PathBounds {
    min_left: usize,
    max_left: usize,
    max_depth: usize,
}

fn get_path_bounds(paths: &[Path]) -> PathBounds {
    let (min_left, max_left, max_depth) = paths
        .iter()
        .flatten()
        .fold((ENTRY_POINT.x, ENTRY_POINT.x, 0), |a, b| {
            (a.0.min(b.x), a.1.max(b.x), a.2.max(b.y))
        });
    PathBounds {
        min_left,
        max_left,
        max_depth,
    }
}

impl From<Vec<Path>> for Cave {
    fn from(paths: Vec<Path>) -> Self {
        let PathBounds {
            min_left,
            max_left,
            max_depth,
        } = get_path_bounds(&paths);
        let mut res = Self::new(min_left, max_left, max_depth);
        let tuples = |path: Vec<Point>| path.into_iter().tuple_windows();
        for (a, b) in paths.into_iter().flat_map(tuples) {
            use std::cmp::Ordering::*;
            let next: fn(Point) -> Point = match (a.x.cmp(&b.x), a.y.cmp(&b.y)) {
                (Less, _) => |p| Point::new(p.x + 1, p.y),
                (Greater, _) => |p| Point::new(p.x - 1, p.y),
                (_, Less) => |p| Point::new(p.x, p.y + 1),
                (_, Greater) => |p| Point::new(p.x, p.y - 1),
                _ => |p| p,
            };
            let mut current = a;
            while current != b {
                res[current] = Space::Rock;
                current = next(current)
            }
            res[current] = Space::Rock;
        }
        res
    }
}

impl Cave {
    fn point_to_internal_index(&self, index: Point) -> usize {
        if !(self.min_left..self.min_left + self.width).contains(&index.x)
            || !(0..self.depth).contains(&index.y)
        {
            panic!("index out of bounds: {index}");
        }
        let x = index.x - self.min_left;
        x + index.y * self.width
    }

    fn new(min_left: usize, max_left: usize, max_depth: usize) -> Self {
        // Add some padding to prevent index out of bounds
        // while a unit of sand is settling.
        let min_left = min_left - 1;
        let width = max_left - min_left + 2;
        let depth = max_depth + 1;
        Self {
            spaces: vec![Default::default(); width * depth],
            min_left,
            width,
            depth,
        }
    }

    /// returns whether the sand successfully came to rest
    fn insert_sand_unit(&mut self) -> bool {
        if self[ENTRY_POINT] == Space::Sand {
            return false;
        }
        let mut sand = ENTRY_POINT;
        while sand.y < self.depth - 1 {
            if [
                Point::new(sand.x, sand.y + 1),
                Point::new(sand.x.saturating_sub(1), sand.y + 1),
                Point::new(sand.x + 1, sand.y + 1),
            ]
            .into_iter()
            .find(|p| self[*p] == Space::Air)
            .map(|p| sand = p)
            .is_none()
            {
                self[sand] = Space::Sand;
                return true;
            }
        }
        false
    }

    fn count_sand_units(&self) -> usize {
        self.spaces.iter().filter(|&&s| s == Space::Sand).count()
    }
}

pub fn part1(mut input: &str) -> usize {
    let paths = parse::paths(&mut input).unwrap();
    let mut cave = Cave::from(paths);
    while cave.insert_sand_unit() { /* println!("{cave}"); */ }
    cave.count_sand_units()
}

pub fn part2(mut input: &str) -> usize {
    let mut paths = parse::paths(&mut input).unwrap();
    let PathBounds { max_depth, .. } = get_path_bounds(&paths);
    let floor_depth = max_depth + 2;
    let floor: Path = vec![
        Point::new(ENTRY_POINT.x - floor_depth, floor_depth),
        Point::new(ENTRY_POINT.x + floor_depth, floor_depth),
    ];
    paths.push(floor);
    let mut cave = Cave::from(paths);
    while cave.insert_sand_unit() { /* println!("{cave}"); */ }
    cave.count_sand_units()
}
