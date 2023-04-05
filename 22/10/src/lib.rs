enum Instruction {
    Addx(i32),
    Noop,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut iter = value.split_ascii_whitespace();
        match iter.next().unwrap() {
            "noop" => Self::Noop,
            "addx" => Self::Addx(iter.next().unwrap().parse().unwrap()),
            instr => panic!("unknown instruction: {instr}"),
        }
    }
}

struct Cpu<F: FnMut(usize, i32)> {
    x: i32,
    cycle: usize,
    debugger: Option<F>,
}

impl<F: FnMut(usize, i32)> Default for Cpu<F> {
    fn default() -> Self {
        Self {
            x: 1,
            cycle: 0,
            debugger: None,
        }
    }
}

impl<F: FnMut(usize, i32)> Cpu<F> {
    fn with_debugger(debugger: F) -> Self {
        Self {
            debugger: Some(debugger),
            ..Default::default()
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;
        if let Some(debugger) = self.debugger.as_mut() {
            debugger(self.cycle, self.x)
        }
    }

    fn exec(&mut self, instr: Instruction) {
        match instr {
            Instruction::Addx(v) => {
                self.tick();
                self.tick();
                self.x += v;
            }
            Instruction::Noop => self.tick(),
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut sig_str_sum = 0;
    let mut cpu = Cpu::with_debugger(|cycle: usize, x: i32| {
        if (cycle + 20) % 40 == 0 {
            sig_str_sum += cycle as i32 * x;
        }
    });
    for instr in input.lines().map(Instruction::from) {
        cpu.exec(instr);
    }
    sig_str_sum
}

pub fn part2(input: &str) -> usize {
    0
}
