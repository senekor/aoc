#[cfg(test)]
mod tests {
    use super::FancyKeypad;

    fn new_keypad(val: u8) -> FancyKeypad {
        FancyKeypad(val)
    }

    #[test]
    fn test_move_left() {
        let mut keypad = new_keypad(1);
        keypad.move_left();
        assert_eq!(new_keypad(1), keypad);

        keypad = new_keypad(0xA);
        keypad.move_left();
        assert_eq!(new_keypad(0xA), keypad);

        keypad = new_keypad(8);
        keypad.move_left();
        assert_eq!(new_keypad(7), keypad);
    }

    #[test]
    fn test_move_right() {
        let mut keypad = new_keypad(1);
        keypad.move_right();
        assert_eq!(new_keypad(1), keypad);

        keypad = new_keypad(0xC);
        keypad.move_right();
        assert_eq!(new_keypad(0xC), keypad);

        keypad = new_keypad(6);
        keypad.move_right();
        assert_eq!(new_keypad(7), keypad);
    }

    #[test]
    fn test_move_up() {
        let mut keypad = new_keypad(1);
        keypad.move_up();
        assert_eq!(new_keypad(1), keypad);

        keypad = new_keypad(9);
        keypad.move_up();
        assert_eq!(new_keypad(9), keypad);

        keypad = new_keypad(0xB);
        keypad.move_up();
        assert_eq!(new_keypad(7), keypad);
    }

    #[test]
    fn test_move_down() {
        let mut keypad = new_keypad(0xD);
        keypad.move_down();
        assert_eq!(new_keypad(0xD), keypad);

        keypad = new_keypad(5);
        keypad.move_down();
        assert_eq!(new_keypad(5), keypad);

        keypad = new_keypad(8);
        keypad.move_down();
        assert_eq!(new_keypad(0xC), keypad);
    }
}

use crate::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FancyKeypad(u8);

impl FancyKeypad {
    fn move_left(&mut self) {
        match self.0 {
            1 | 2 | 5 | 0xA | 0xD => {}
            _ => self.0 -= 1,
        }
    }
    fn move_right(&mut self) {
        match self.0 {
            1 | 4 | 9 | 0xC | 0xD => {}
            _ => self.0 += 1,
        }
    }
    fn move_up(&mut self) {
        match self.0 {
            3 | 0xD => self.0 -= 2,
            6..=8 | 0xA..=0xC => self.0 -= 4,
            _ => {}
        }
    }
    fn move_down(&mut self) {
        match self.0 {
            1 | 0xB => self.0 += 2,
            2..=4 | 6..=8 => self.0 += 4,
            _ => {}
        }
    }
}

impl super::Keypad for FancyKeypad {
    fn change_pos(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
        }
    }

    fn get_pos(&self) -> u8 {
        self.0
    }

    fn get_base(&self) -> u32 {
        16
    }
}

impl Default for FancyKeypad {
    fn default() -> Self {
        FancyKeypad(5)
    }
}
