#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    num_steps: usize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let num_steps = value[2..].parse().unwrap();
        let direction = match &value[..1] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            d => panic!("unknown direction: {d}"),
        };
        Self {
            direction,
            num_steps,
        }
    }
}

#[derive(Debug, Default)]
struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
}

impl Rope {
    fn pull_tail(&mut self) {
        let x_diff = (self.head.0) - (self.tail.0);
        let y_diff = (self.head.1) - (self.tail.1);
        let offset = match (x_diff, y_diff) {
            (2, 0) => (1, 0),
            (2, 1) => (1, 1),
            (1, 2) => (1, 1),
            (0, 2) => (0, 1),
            (-1, 2) => (-1, 1),
            (-2, 1) => (-1, 1),
            (-2, 0) => (-1, 0),
            (-2, -1) => (-1, -1),
            (-1, -2) => (-1, -1),
            (0, -2) => (0, -1),
            (1, -2) => (1, -1),
            (2, -1) => (1, -1),
            _ => (0, 0),
        };
        self.tail.0 += offset.0;
        self.tail.1 += offset.1;
    }
}

#[derive(Debug)]
struct TailPositions<T: Iterator<Item = Instruction>> {
    rope: Rope,
    instructions: T,
    partial_instr: Option<Instruction>,
}

impl<T: Iterator<Item = Instruction>> TailPositions<T> {
    fn new(iter: impl IntoIterator<IntoIter = T>) -> Self {
        Self {
            rope: Default::default(),
            instructions: iter.into_iter(),
            partial_instr: None,
        }
    }
}

impl<T: Iterator<Item = Instruction>> Iterator for TailPositions<T> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let instr = self.partial_instr.or_else(|| self.instructions.next())?;

        self.partial_instr = if instr.num_steps == 1 {
            None
        } else {
            Some(Instruction {
                direction: instr.direction,
                num_steps: instr.num_steps - 1,
            })
        };
        match instr.direction {
            Direction::Right => self.rope.head.0 += 1,
            Direction::Left => self.rope.head.0 -= 1,
            Direction::Up => self.rope.head.1 += 1,
            Direction::Down => self.rope.head.1 -= 1,
        }
        self.rope.pull_tail();
        Some(self.rope.tail)
    }
}

pub fn part1(input: &str) -> usize {
    let mut positions =
        TailPositions::new(input.lines().map(Instruction::from)).collect::<Vec<_>>();
    positions.sort();
    positions.dedup();
    positions.len()
}

pub fn part2(input: &str) -> usize {
    0
}

// #[cfg(test)]
// mod unit_tests {
//     use super::*;

//     #[test]
//     fn sanity_check() {
//         assert_eq!(1 + 1, 2)
//     }
// }
