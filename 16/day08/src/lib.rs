#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Rect { w: usize, h: usize },
    RotateRow { row: usize, by: usize },
    RotateCol { col: usize, by: usize },
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut iter = s.split(&[' ', 'x', 'y', '=']);

        if iter.next() == Some("rect") {
            return Self::Rect {
                w: iter.next().unwrap().parse().unwrap(),
                h: iter.next().unwrap().parse().unwrap(),
            };
        }

        match iter.next() {
            Some("row") => Self::RotateRow {
                row: iter.nth(2).unwrap().parse().unwrap(),
                by: iter.nth(2).unwrap().parse().unwrap(),
            },
            _ => Self::RotateCol {
                col: iter.nth(2).unwrap().parse().unwrap(),
                by: iter.nth(2).unwrap().parse().unwrap(),
            },
        }
    }
}

#[test]
fn test_instr_from_str() {
    assert_eq!(
        Instruction::from("rect 3x2"),
        Instruction::Rect { w: 3, h: 2 }
    );
    assert_eq!(
        Instruction::from("rotate row y=0 by 4"),
        Instruction::RotateRow { row: 0, by: 4 }
    );
    assert_eq!(
        Instruction::from("rotate column x=1 by 1"),
        Instruction::RotateCol { col: 1, by: 1 }
    )
}

struct Keycard {
    mag_strip_instructions: Vec<Instruction>,
}

impl From<&str> for Keycard {
    fn from(s: &str) -> Self {
        Self {
            mag_strip_instructions: s.lines().map(Instruction::from).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    On,
    Off,
}

impl Default for Pixel {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(Debug)]
struct Screen {
    pixels: [[Pixel; 50]; 6],
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            pixels: [[Pixel::default(); 50]; 6],
        }
    }
}

impl Screen {
    fn exec_rect(&mut self, width: usize, height: usize) {
        for x in 0..width {
            for y in 0..height {
                self.pixels[y][x] = Pixel::On;
            }
        }
    }

    fn exec_rotate_row(&mut self, row: usize, by: usize) {
        let mut new_row = [Pixel::default(); 50];
        for (x, new_pixel) in new_row.iter_mut().enumerate() {
            *new_pixel = self.pixels[row][(x + 50 - by) % 50]
        }
        for (x, p) in new_row.into_iter().enumerate() {
            self.pixels[row][x] = p
        }
    }

    fn exec_rotate_col(&mut self, col: usize, by: usize) {
        let mut new_col = [Pixel::default(); 6];
        for (y, new_pixel) in new_col.iter_mut().enumerate() {
            *new_pixel = self.pixels[(y + 6 - by) % 6][col]
        }
        for (y, p) in new_col.into_iter().enumerate() {
            self.pixels[y][col] = p
        }
    }

    fn exec(&mut self, instr: Instruction) {
        match instr {
            Instruction::Rect { w, h } => self.exec_rect(w, h),
            Instruction::RotateRow { row, by } => self.exec_rotate_row(row, by),
            Instruction::RotateCol { col, by } => self.exec_rotate_col(col, by),
        }
    }

    fn exec_all(&mut self, instructions: Vec<Instruction>) {
        instructions.into_iter().for_each(|instr| self.exec(instr))
    }

    fn count_pixels(&self) -> usize {
        self.pixels
            .iter()
            .flat_map(|col| col.iter())
            .filter(|p| **p == Pixel::On)
            .count()
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = self
            .pixels
            .iter()
            .map(|col| {
                col.iter()
                    .map(|p| if *p == Pixel::On { '#' } else { ' ' })
                    .collect::<String>()
                    + "\n"
            })
            .collect::<String>();
        write!(f, "{}", res)
    }
}

pub fn part1(input: &str) -> usize {
    let keycard = Keycard::from(input);
    let mut screen = Screen::default();
    screen.exec_all(keycard.mag_strip_instructions);
    screen.count_pixels()
}

pub fn part2(input: &str) -> String {
    let keycard = Keycard::from(input);
    let mut screen = Screen::default();
    screen.exec_all(keycard.mag_strip_instructions);
    screen.to_string()
}
