use std::collections::HashSet;

enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut iter = s.split_whitespace();
        let instr = iter.next().unwrap();
        let num = iter.next().unwrap().parse().unwrap();
        match instr {
            "acc" => Self::Acc(num),
            "jmp" => Self::Jmp(num),
            "nop" => Self::Nop(num),
            x => panic!("unknown instruction {x}"),
        }
    }
}

struct BootCode(Vec<Instruction>);

impl From<&str> for BootCode {
    fn from(s: &str) -> Self {
        Self(s.lines().map(Instruction::from).collect())
    }
}

struct Executor<'a> {
    accumulator: i32,
    boot_code: &'a BootCode,
    instr_ptr: usize,
    visited: HashSet<usize>,
}

impl<'a> Executor<'a> {
    fn new(boot_code: &'a BootCode) -> Executor<'a> {
        Self {
            accumulator: 0,
            boot_code,
            instr_ptr: 0,
            visited: HashSet::new(),
        }
    }
}

impl Executor<'_> {
    fn add_to_instr_ptr(&mut self, i: i32) {
        if i.is_negative() {
            self.instr_ptr -= (-i) as usize;
        } else {
            self.instr_ptr += i as usize;
        }
    }

    fn execute_once(&mut self) {
        match self.boot_code.0[self.instr_ptr] {
            Instruction::Acc(i) => {
                self.accumulator += i;
                self.instr_ptr += 1
            }
            Instruction::Jmp(i) => self.add_to_instr_ptr(i),
            Instruction::Nop(_) => self.instr_ptr += 1,
        };
    }

    fn find_infinite_loop(&mut self) -> i32 {
        while !self.visited.contains(&self.instr_ptr) {
            self.visited.insert(self.instr_ptr);
            self.execute_once();
        }
        self.accumulator
    }

    fn boot_code_is_fixed(&mut self) -> Option<i32> {
        while !self.visited.contains(&self.instr_ptr) {
            self.visited.insert(self.instr_ptr);
            self.execute_once();
            if self.instr_ptr == self.boot_code.0.len() {
                return Some(self.accumulator);
            }
        }
        None
    }
}

pub fn part1(input: &str) -> i32 {
    Executor::new(&BootCode::from(input)).find_infinite_loop()
}

pub fn part2(input: &str) -> i32 {
    let mut boot_code = BootCode::from(input);
    for i in 0..boot_code.0.len() {
        match boot_code.0[i] {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(num) => boot_code.0[i] = Instruction::Nop(num),
            Instruction::Nop(num) => boot_code.0[i] = Instruction::Jmp(num),
        };
        if let Some(res) = Executor::new(&boot_code).boot_code_is_fixed() {
            return res;
        }
        match boot_code.0[i] {
            Instruction::Acc(_) => {}
            Instruction::Jmp(num) => boot_code.0[i] = Instruction::Nop(num),
            Instruction::Nop(num) => boot_code.0[i] = Instruction::Jmp(num),
        };
    }
    panic!("not found")
}
