#[derive(Debug, Clone, Default)]
struct Executor {
    idx: usize,
    instr: Vec<i32>,
    part2: bool,
}

impl Iterator for Executor {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.instr.len() <= self.idx {
            return None;
        }
        let prev_idx = self.idx;
        let offset = self.instr[self.idx];
        if offset < 0 {
            let neg = -offset as usize;
            if self.idx < neg {
                return None;
            }
            self.idx -= neg;
        } else {
            self.idx += offset as usize;
        }
        if self.part2 && offset >= 3 {
            self.instr[prev_idx] -= 1;
        } else {
            self.instr[prev_idx] += 1;
        }
        Some(self.idx)
    }
}

impl<T> From<T> for Executor
where
    T: IntoIterator<Item = i32>,
{
    fn from(instr: T) -> Self {
        Self {
            instr: instr.into_iter().collect(),
            ..Self::default()
        }
    }
}

pub fn part1(input: &str) -> usize {
    Executor::from(input.lines().map(|l| l.parse().unwrap())).count()
}

pub fn part2(input: &str) -> usize {
    let mut exec = Executor::from(input.lines().map(|l| l.parse().unwrap()));
    exec.part2 = true;
    exec.count()
}
