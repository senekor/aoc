struct Ship {
    crates: Vec<Vec<char>>,
}

impl From<&str> for Ship {
    fn from(value: &str) -> Self {
        let mut lines = value.lines().collect::<Vec<_>>();
        lines.pop(); // drop line with numbers
        lines.reverse();
        let num_crates = lines[0].len() / 4 + 1;

        let mut crates = vec![vec![]; num_crates];
        for line in lines {
            for (offset, stack) in crates.iter_mut().enumerate().map(|(i, s)| (i * 4 + 1, s)) {
                match line.as_bytes().get(offset) {
                    Some(b' ') | None => {}
                    Some(&c) => stack.push(c as char),
                }
            }
        }
        Self { crates }
    }
}

struct Instruction {
    num_moved: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let nums = value
            .split_whitespace()
            .flat_map(|s| s.parse().ok())
            .collect::<Vec<_>>();
        Self {
            num_moved: nums[0],
            from: nums[1] - 1,
            to: nums[2] - 1,
        }
    }
}

fn parse_input(input: &str) -> (Ship, Vec<Instruction>) {
    let (ship, instructions) = input.split_once("\n\n").unwrap();
    (
        Ship::from(ship),
        instructions
            .lines()
            .map(Instruction::from)
            .collect::<Vec<_>>(),
    )
}

impl Ship {
    fn exec(&mut self, instruction: Instruction) {
        for _ in 0..instruction.num_moved {
            let grabbed = self.crates[instruction.from].pop().unwrap();
            self.crates[instruction.to].push(grabbed);
        }
    }

    fn exec_all(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            self.exec(instruction);
        }
    }

    fn top_crates(&self) -> String {
        self.crates
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect()
    }

    fn exec_2(&mut self, instruction: Instruction) {
        let mut grabbed = Vec::with_capacity(instruction.num_moved);
        for _ in 0..instruction.num_moved {
            grabbed.push(self.crates[instruction.from].pop().unwrap())
        }
        for _ in 0..instruction.num_moved {
            self.crates[instruction.to].push(grabbed.pop().unwrap())
        }
    }

    fn exec_all_2(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            self.exec_2(instruction);
        }
    }
}

pub fn part1(input: &str) -> String {
    let (mut ship, instructions) = parse_input(input);
    ship.exec_all(instructions);
    ship.top_crates()
}

pub fn part2(input: &str) -> String {
    let (mut ship, instructions) = parse_input(input);
    ship.exec_all_2(instructions);
    ship.top_crates()
}

// #[cfg(test)]
// mod unit_tests {
//     use super::*;

//     #[test]
//     fn sanity_check() {
//         assert_eq!(1 + 1, 2)
//     }
// }
