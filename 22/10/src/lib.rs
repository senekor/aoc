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

fn display_screen(screen: [[bool; 40]; 6]) -> String {
    screen
        .into_iter()
        .map(|row| {
            std::iter::once('\n')
                .chain(row.into_iter().map(|pixel| if pixel { '#' } else { '.' }))
                .collect::<String>()
        })
        .collect::<String>()
}

pub fn part2(input: &str) -> String {
    let mut screen = [[false; 40]; 6];
    let mut cpu = Cpu::with_debugger(|cycle: usize, x: i32| {
        let row = (cycle / 40) % 6;
        let col = (cycle - 1) % 40;
        let pos = col as i32;
        if x - 1 <= pos && x + 1 >= pos {
            screen[row][col] = true;
        }
    });
    for instr in input.lines().map(Instruction::from) {
        cpu.exec(instr);
    }
    display_screen(screen)
}
