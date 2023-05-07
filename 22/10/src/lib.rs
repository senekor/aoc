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

trait CpuObserver {
    fn notify(&mut self, cycle: usize, x: i32);
}

struct Cpu<'a> {
    x: i32,
    cycle: usize,
    observers: Vec<&'a mut dyn CpuObserver>,
}

impl Default for Cpu<'_> {
    fn default() -> Self {
        Self {
            x: 1,
            cycle: 0,
            observers: Vec::new(),
        }
    }
}

impl<'a> Cpu<'a> {
    fn register_observer(&mut self, obs: &'a mut dyn CpuObserver) {
        self.observers.push(obs);
    }

    fn tick(&mut self) {
        self.cycle += 1;
        for observer in self.observers.iter_mut() {
            observer.notify(self.cycle, self.x);
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

#[derive(Debug, Default)]
struct SignalStrength {
    sum: i32,
}

impl CpuObserver for SignalStrength {
    fn notify(&mut self, cycle: usize, x: i32) {
        if (cycle + 20) % 40 == 0 {
            self.sum += cycle as i32 * x;
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut cpu = Cpu::default();
    let mut signal_strength = SignalStrength::default();
    cpu.register_observer(&mut signal_strength);
    for instr in input.lines().map(Instruction::from) {
        cpu.exec(instr);
    }
    signal_strength.sum
}

#[derive(Debug)]
struct Monitor {
    screen: [[bool; 40]; 6],
}

impl Default for Monitor {
    fn default() -> Self {
        Self {
            screen: [[false; 40]; 6],
        }
    }
}

struct PixelLoc {
    row: usize,
    col: usize,
}

impl Monitor {
    fn draw_pixel(&mut self, pixel_loc: PixelLoc, sprite_loc: i32) {
        if (sprite_loc - 1..=sprite_loc + 1).contains(&(pixel_loc.col as i32)) {
            self.screen[pixel_loc.row][pixel_loc.col] = true;
        }
    }

    fn display_screen(&self) -> String {
        self.screen
            .into_iter()
            .flat_map(|row| {
                std::iter::once('\n')
                    .chain(row.into_iter().map(|pixel| if pixel { '#' } else { '.' }))
            })
            .collect::<String>()
    }
}

impl CpuObserver for Monitor {
    fn notify(&mut self, cycle: usize, x: i32) {
        let pixel_loc = PixelLoc {
            row: (cycle / 40) % 6,
            col: (cycle - 1) % 40,
        };
        self.draw_pixel(pixel_loc, x)
    }
}

pub fn part2(input: &str) -> String {
    let mut cpu = Cpu::default();
    let mut monitor = Monitor::default();
    cpu.register_observer(&mut monitor);
    for instr in input.lines().map(Instruction::from) {
        cpu.exec(instr);
    }
    monitor.display_screen()
}
