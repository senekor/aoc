//! Note that 19-07 builds on this, some of the functionality here is only
//! needed there.

use std::collections::VecDeque;

enum Mode {
    Position,
    Immediate,
}

impl From<i32> for Mode {
    fn from(mode: i32) -> Self {
        match mode {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("unknown parameter mode"),
        }
    }
}

struct SrcParam(i32, Mode);

type DestParam = usize;

enum Instruction {
    Add {
        src_1: SrcParam,
        src_2: SrcParam,
        dest: DestParam,
    },
    Mult {
        src_1: SrcParam,
        src_2: SrcParam,
        dest: DestParam,
    },
    Read(DestParam),
    Write(SrcParam),
    JumpTrue(SrcParam, SrcParam),
    JumpFalse(SrcParam, SrcParam),
    LessThan {
        src_1: SrcParam,
        src_2: SrcParam,
        dest: DestParam,
    },
    Equals {
        src_1: SrcParam,
        src_2: SrcParam,
        dest: DestParam,
    },
    Stop,
}

#[derive(Debug, Clone)]
pub struct Program {
    instr_ptr: usize,
    data: Vec<i32>,
    inputs: VecDeque<i32>,
}

impl std::str::FromStr for Program {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Program {
            instr_ptr: 0,
            data: s.split(',').map(|x| x.parse().unwrap()).collect(),
            inputs: VecDeque::new(),
        })
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data[0]).unwrap();
        for x in &self.data[1..] {
            write!(f, ",{}", x).unwrap();
        }
        Ok(())
    }
}

pub enum InstrOutput {
    Nothing,
    Something(i32),
    Stop,
}

impl Program {
    fn get_instr(&self) -> Instruction {
        let instr = self.data[self.instr_ptr];
        let opcode = instr % 100;
        let mode_1 = Mode::from(instr / 100 % 10);
        let mode_2 = Mode::from(instr / 1_000 % 10);
        match opcode {
            1 => Instruction::Add {
                src_1: SrcParam(self.data[self.instr_ptr + 1], mode_1),
                src_2: SrcParam(self.data[self.instr_ptr + 2], mode_2),
                dest: self.data[self.instr_ptr + 3] as usize,
            },
            2 => Instruction::Mult {
                src_1: SrcParam(self.data[self.instr_ptr + 1], mode_1),
                src_2: SrcParam(self.data[self.instr_ptr + 2], mode_2),
                dest: self.data[self.instr_ptr + 3] as usize,
            },
            3 => Instruction::Read(self.data[self.instr_ptr + 1] as usize),
            4 => Instruction::Write(SrcParam(self.data[self.instr_ptr + 1], mode_1)),
            5 => Instruction::JumpTrue(
                SrcParam(self.data[self.instr_ptr + 1], mode_1),
                SrcParam(self.data[self.instr_ptr + 2], mode_2),
            ),
            6 => Instruction::JumpFalse(
                SrcParam(self.data[self.instr_ptr + 1], mode_1),
                SrcParam(self.data[self.instr_ptr + 2], mode_2),
            ),
            7 => Instruction::LessThan {
                src_1: SrcParam(self.data[self.instr_ptr + 1], mode_1),
                src_2: SrcParam(self.data[self.instr_ptr + 2], mode_2),
                dest: self.data[self.instr_ptr + 3] as usize,
            },
            8 => Instruction::Equals {
                src_1: SrcParam(self.data[self.instr_ptr + 1], mode_1),
                src_2: SrcParam(self.data[self.instr_ptr + 2], mode_2),
                dest: self.data[self.instr_ptr + 3] as usize,
            },
            99 => Instruction::Stop,
            _ => panic!("unknown opcode"),
        }
    }

    fn get_val(&self, param: SrcParam) -> i32 {
        match param.1 {
            Mode::Position => self.data[param.0 as usize],
            Mode::Immediate => param.0,
        }
    }

    /// returns the output in case of an output instruction
    pub fn execute_one_instr(&mut self) -> InstrOutput {
        match self.get_instr() {
            Instruction::Add { src_1, src_2, dest } => {
                self.data[dest] = self.get_val(src_1) + self.get_val(src_2);
                self.instr_ptr += 4;
            }
            Instruction::Mult { src_1, src_2, dest } => {
                self.data[dest] = self.get_val(src_1) * self.get_val(src_2);
                self.instr_ptr += 4;
            }
            Instruction::Read(dest) => {
                self.data[dest] = self.inputs.pop_front().unwrap();
                self.instr_ptr += 2;
            }
            Instruction::Write(src) => {
                let val = self.get_val(src);
                self.instr_ptr += 2;
                return InstrOutput::Something(val);
            }
            Instruction::JumpTrue(src_1, src_2) => {
                if self.get_val(src_1) != 0 {
                    self.instr_ptr = self.get_val(src_2) as usize
                } else {
                    self.instr_ptr += 3
                }
            }
            Instruction::JumpFalse(src_1, src_2) => {
                if self.get_val(src_1) == 0 {
                    self.instr_ptr = self.get_val(src_2) as usize
                } else {
                    self.instr_ptr += 3
                }
            }
            Instruction::LessThan { src_1, src_2, dest } => {
                self.data[dest] = i32::from(self.get_val(src_1) < self.get_val(src_2));
                self.instr_ptr += 4;
            }
            Instruction::Equals { src_1, src_2, dest } => {
                self.data[dest] = i32::from(self.get_val(src_1) == self.get_val(src_2));
                self.instr_ptr += 4;
            }
            Instruction::Stop => return InstrOutput::Stop,
        }
        InstrOutput::Nothing
    }

    fn execute(&mut self) -> Vec<i32> {
        let mut output = Vec::new();
        loop {
            match self.execute_one_instr() {
                InstrOutput::Nothing => {}
                InstrOutput::Something(val) => output.push(val),
                InstrOutput::Stop => return output,
            }
        }
    }

    pub fn reset(&mut self) {
        self.instr_ptr = 0;
        self.inputs.clear();
    }

    pub fn add_input(&mut self, n: i32) {
        self.inputs.push_back(n)
    }
}

fn test(input: &str, n: i32) -> i32 {
    let mut prog: Program = input.parse().unwrap();
    prog.add_input(n);
    let output = prog.execute();
    assert!(
        output[..output.len() - 1].iter().all(|&x| x == 0),
        "all output except the last must be zero"
    );
    output.last().unwrap().to_owned()
}

pub fn part1(input: &str) -> i32 {
    test(input, 1)
}

pub fn part2(input: &str) -> i32 {
    test(input, 5)
}
