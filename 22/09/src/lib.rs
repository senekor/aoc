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

#[derive(Debug)]
struct Rope<const N: usize> {
    knots: [(i32, i32); N],
}

impl<const N: usize> Default for Rope<N> {
    fn default() -> Self {
        Self { knots: [(0, 0); N] }
    }
}

impl<const N: usize> Rope<N> {
    fn pull_knots(&mut self) {
        for i in 1..N {
            let section = &mut self.knots[i - 1..=i];
            let x_diff = (section[0].0) - (section[1].0);
            let y_diff = (section[0].1) - (section[1].1);
            let offset = match (x_diff, y_diff) {
                (2, 0) => (1, 0),
                (2, 1) => (1, 1),
                (2, 2) => (1, 1),
                (1, 2) => (1, 1),
                (0, 2) => (0, 1),
                (-1, 2) => (-1, 1),
                (-2, 2) => (-1, 1),
                (-2, 1) => (-1, 1),
                (-2, 0) => (-1, 0),
                (-2, -1) => (-1, -1),
                (-2, -2) => (-1, -1),
                (-1, -2) => (-1, -1),
                (0, -2) => (0, -1),
                (1, -2) => (1, -1),
                (2, -2) => (1, -1),
                (2, -1) => (1, -1),
                _ => (0, 0),
            };
            section[1].0 += offset.0;
            section[1].1 += offset.1;
        }
    }
}

#[derive(Debug)]
struct TailPositions<T: Iterator<Item = Instruction>, const N: usize> {
    rope: Rope<N>,
    instructions: T,
    partial_instr: Option<Instruction>,
}

impl<T: Iterator<Item = Instruction>, const N: usize> TailPositions<T, N> {
    fn new(iter: impl IntoIterator<IntoIter = T>) -> Self {
        Self {
            rope: Default::default(),
            instructions: iter.into_iter(),
            partial_instr: None,
        }
    }
}

impl<T: Iterator<Item = Instruction>, const N: usize> Iterator for TailPositions<T, N> {
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
            Direction::Right => self.rope.knots[0].0 += 1,
            Direction::Left => self.rope.knots[0].0 -= 1,
            Direction::Up => self.rope.knots[0].1 += 1,
            Direction::Down => self.rope.knots[0].1 -= 1,
        }
        self.rope.pull_knots();
        Some(self.rope.knots[N - 1])
    }
}

fn pull_rope<const N: usize>(input: &str) -> usize {
    let mut positions =
        TailPositions::<_, N>::new(input.lines().map(Instruction::from)).collect::<Vec<_>>();
    positions.sort();
    positions.dedup();
    positions.len()
}

pub fn part1(input: &str) -> usize {
    pull_rope::<2>(input)
}

pub fn part2(input: &str) -> usize {
    pull_rope::<10>(input)
}
