use std::ops::{Index, IndexMut};

use strum::EnumString;

#[derive(Clone, Copy, EnumString)]
enum Register {
    #[strum(ascii_case_insensitive)]
    A,
    #[strum(ascii_case_insensitive)]
    B,
}

type Offset = i32;

#[derive(Clone)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(Offset),
    JumpIfEven(Register, Offset),
    JumpIfOne(Register, Offset),
}

impl Instruction {
    fn is_jump(&self) -> bool {
        use Instruction::*;
        matches!(self, Jump(_) | JumpIfEven(_, _) | JumpIfOne(_, _))
    }
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut iter = s.split_ascii_whitespace();
        let instr = iter.next().unwrap();
        let arg0 = iter.next().unwrap().trim_end_matches(',');
        let arg1 = iter.next();
        match instr {
            "hlf" => Self::Half(arg0.parse().unwrap()),
            "tpl" => Self::Triple(arg0.parse().unwrap()),
            "inc" => Self::Increment(arg0.parse().unwrap()),
            "jmp" => Self::Jump(arg0.parse().unwrap()),
            "jie" => Self::JumpIfEven(arg0.parse().unwrap(), arg1.unwrap().parse().unwrap()),
            "jio" => Self::JumpIfOne(arg0.parse().unwrap(), arg1.unwrap().parse().unwrap()),
            s => panic!("unknown instruction {s}"),
        }
    }
}

#[derive(Default)]
struct Computer {
    a: u32,
    b: u32,
    instructions: Vec<Instruction>,
    instr_ptr: usize,
}

impl Computer {
    fn jmp(&mut self, offset: i32, condition: bool) {
        if condition {
            if offset.is_negative() {
                self.instr_ptr -= offset.unsigned_abs() as usize;
            } else {
                self.instr_ptr += offset as usize;
            }
        } else {
            self.instr_ptr += 1
        }
    }

    fn exec_one(&mut self) {
        use Instruction::*;
        let instr = self.instructions[self.instr_ptr].clone();
        match instr {
            Half(r) => self[r] /= 2,
            Triple(r) => self[r] *= 3,
            Increment(r) => self[r] += 1,
            Jump(o) => self.jmp(o, true),
            JumpIfEven(r, o) => self.jmp(o, self[r] % 2 == 0),
            JumpIfOne(r, o) => self.jmp(o, self[r] == 1),
        };
        if !instr.is_jump() {
            self.instr_ptr += 1
        }
    }

    fn exec_program(&mut self) {
        while self.instr_ptr < self.instructions.len() {
            self.exec_one()
        }
    }
}

impl Index<Register> for Computer {
    type Output = u32;
    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::A => &self.a,
            Register::B => &self.b,
        }
    }
}

impl IndexMut<Register> for Computer {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }
}

impl From<&str> for Computer {
    fn from(s: &str) -> Self {
        Self {
            instructions: s.lines().map(Instruction::from).collect(),
            ..Default::default()
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut computer = Computer::from(input);
    computer.exec_program();
    computer.b
}

pub fn part2(input: &str) -> u32 {
    let mut computer = Computer::from(input);
    computer.a = 1;
    computer.exec_program();
    computer.b
}
