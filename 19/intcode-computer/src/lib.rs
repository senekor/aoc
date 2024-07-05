use std::collections::VecDeque;

mod memory;

enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for Mode {
    fn from(mode: i64) -> Self {
        match mode {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("unknown parameter mode"),
        }
    }
}

struct Param(i64, Mode);

enum Instruction {
    Add {
        src_1: Param,
        src_2: Param,
        dest: Param,
    },
    Mult {
        src_1: Param,
        src_2: Param,
        dest: Param,
    },
    Read(Param),
    Write(Param),
    JumpTrue(Param, Param),
    JumpFalse(Param, Param),
    LessThan {
        src_1: Param,
        src_2: Param,
        dest: Param,
    },
    Equals {
        src_1: Param,
        src_2: Param,
        dest: Param,
    },
    AdjustRelativeBase(Param),
    Stop,
}

#[derive(Debug, Clone)]
pub struct IntcodeComputer {
    instr_ptr: usize,
    relative_base: i64,
    memory: memory::Memory,
    inputs: VecDeque<i64>,
}

impl std::str::FromStr for IntcodeComputer {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(IntcodeComputer {
            instr_ptr: 0,
            relative_base: 0,
            memory: memory::Memory::from(s),
            inputs: VecDeque::new(),
        })
    }
}

impl std::fmt::Display for IntcodeComputer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.memory[0]).unwrap();
        for x in self.memory.iter().skip(1) {
            write!(f, ",{}", x).unwrap();
        }
        Ok(())
    }
}

pub enum InstrOutput {
    Nothing,
    Something(i64),
    Stop,
}

impl IntcodeComputer {
    fn get_instr(&self) -> Instruction {
        let instr = self.memory[self.instr_ptr];
        let opcode = instr % 100;
        let mode_1 = Mode::from(instr / 100 % 10);
        let mode_2 = Mode::from(instr / 1_000 % 10);
        let mode_3 = Mode::from(instr / 10_000 % 10);
        match opcode {
            1 => Instruction::Add {
                src_1: Param(self.memory[self.instr_ptr + 1], mode_1),
                src_2: Param(self.memory[self.instr_ptr + 2], mode_2),
                dest: Param(self.memory[self.instr_ptr + 3], mode_3),
            },
            2 => Instruction::Mult {
                src_1: Param(self.memory[self.instr_ptr + 1], mode_1),
                src_2: Param(self.memory[self.instr_ptr + 2], mode_2),
                dest: Param(self.memory[self.instr_ptr + 3], mode_3),
            },
            3 => Instruction::Read(Param(self.memory[self.instr_ptr + 1], mode_1)),
            4 => Instruction::Write(Param(self.memory[self.instr_ptr + 1], mode_1)),
            5 => Instruction::JumpTrue(
                Param(self.memory[self.instr_ptr + 1], mode_1),
                Param(self.memory[self.instr_ptr + 2], mode_2),
            ),
            6 => Instruction::JumpFalse(
                Param(self.memory[self.instr_ptr + 1], mode_1),
                Param(self.memory[self.instr_ptr + 2], mode_2),
            ),
            7 => Instruction::LessThan {
                src_1: Param(self.memory[self.instr_ptr + 1], mode_1),
                src_2: Param(self.memory[self.instr_ptr + 2], mode_2),
                dest: Param(self.memory[self.instr_ptr + 3], mode_3),
            },
            8 => Instruction::Equals {
                src_1: Param(self.memory[self.instr_ptr + 1], mode_1),
                src_2: Param(self.memory[self.instr_ptr + 2], mode_2),
                dest: Param(self.memory[self.instr_ptr + 3], mode_3),
            },
            9 => Instruction::AdjustRelativeBase(Param(self.memory[self.instr_ptr + 1], mode_1)),
            99 => Instruction::Stop,
            _ => panic!("unknown opcode"),
        }
    }

    fn get_val(&self, param: Param) -> i64 {
        match param.1 {
            Mode::Position => self.memory[param.0 as usize],
            Mode::Immediate => param.0,
            Mode::Relative => self.memory[(self.relative_base + param.0) as usize],
        }
    }
    fn get_dest(&mut self, dest: Param) -> &mut i64 {
        match dest.1 {
            Mode::Position => &mut self.memory[dest.0 as usize],
            Mode::Immediate => panic!("destination parameters cannot be in immediate mode"),
            Mode::Relative => &mut self.memory[(self.relative_base + dest.0) as usize],
        }
    }

    /// returns the output in case of an output instruction
    pub fn execute_one_instr(&mut self) -> InstrOutput {
        match self.get_instr() {
            Instruction::Add { src_1, src_2, dest } => {
                *self.get_dest(dest) = self.get_val(src_1) + self.get_val(src_2);
                self.instr_ptr += 4;
            }
            Instruction::Mult { src_1, src_2, dest } => {
                *self.get_dest(dest) = self.get_val(src_1) * self.get_val(src_2);
                self.instr_ptr += 4;
            }
            Instruction::Read(dest) => {
                *self.get_dest(dest) = self.inputs.pop_front().unwrap();
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
                *self.get_dest(dest) = i64::from(self.get_val(src_1) < self.get_val(src_2));
                self.instr_ptr += 4;
            }
            Instruction::Equals { src_1, src_2, dest } => {
                *self.get_dest(dest) = i64::from(self.get_val(src_1) == self.get_val(src_2));
                self.instr_ptr += 4;
            }
            Instruction::AdjustRelativeBase(param) => {
                self.relative_base += self.get_val(param);
                self.instr_ptr += 2
            }
            Instruction::Stop => return InstrOutput::Stop,
        }
        InstrOutput::Nothing
    }

    pub fn execute(&mut self) -> Vec<i64> {
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

    pub fn add_input(&mut self, n: i64) {
        self.inputs.push_back(n)
    }

    /// # Safety
    ///
    /// This is not really unsafe, but it's fun to pretend.
    pub unsafe fn get_raw_memory(&mut self) -> &mut [i64] {
        self.memory.as_mut_slice()
    }
}
