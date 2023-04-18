use std::collections::HashMap;

#[derive(Debug, Default)]
struct State<'a> {
    registers: HashMap<&'a str, i32>,
}

impl<'a> State<'a> {
    fn exec(&mut self, instr: &'a str) {
        let mut iter = instr.split_ascii_whitespace();
        let target = iter.next().unwrap();
        let operation = iter.next().unwrap();
        let value: i32 = iter.next().unwrap().parse().unwrap();
        assert_eq!(iter.next().unwrap(), "if");
        let to_compare = self
            .registers
            .get(iter.next().unwrap())
            .copied()
            .unwrap_or_default();
        let comparator = iter.next().unwrap();
        let val_comp = iter.next().unwrap().parse().unwrap();
        let comparison_result = match comparator {
            "<" => to_compare < val_comp,
            ">" => to_compare > val_comp,
            "<=" => to_compare <= val_comp,
            ">=" => to_compare >= val_comp,
            "==" => to_compare == val_comp,
            "!=" => to_compare != val_comp,
            _ => panic!(),
        };
        if !comparison_result {
            return;
        }
        let target = self.registers.entry(target).or_default();
        match operation {
            "inc" => *target += value,
            "dec" => *target -= value,
            _ => panic!(),
        }
    }

    fn max(&self) -> i32 {
        self.registers.values().copied().max().unwrap_or_default()
    }
}

pub fn part1(input: &str) -> i32 {
    let mut state = State::default();
    input.lines().for_each(|line| state.exec(line));
    state.max()
}

pub fn part2(input: &str) -> i32 {
    let mut state = State::default();
    input
        .lines()
        .map(|line| {
            state.exec(line);
            state.max()
        })
        .max()
        .unwrap()
}
