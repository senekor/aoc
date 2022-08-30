#[cfg(test)]
mod tests {
    use super::NormalKeypad;

    fn new_keypad(val: u8) -> NormalKeypad {
        NormalKeypad(val)
    }

    #[test]
    fn test_move_left() {
        let mut keypad = new_keypad(1);
        keypad.move_left();
        assert_eq!(new_keypad(1), keypad);

        keypad = new_keypad(7);
        keypad.move_left();
        assert_eq!(new_keypad(7), keypad);

        keypad = new_keypad(6);
        keypad.move_left();
        assert_eq!(new_keypad(5), keypad);
    }

    #[test]
    fn test_move_right() {
        let mut keypad = new_keypad(3);
        keypad.move_right();
        assert_eq!(new_keypad(3), keypad);

        keypad = new_keypad(9);
        keypad.move_right();
        assert_eq!(new_keypad(9), keypad);

        keypad = new_keypad(4);
        keypad.move_right();
        assert_eq!(new_keypad(5), keypad);
    }

    #[test]
    fn test_move_up() {
        let mut keypad = new_keypad(1);
        keypad.move_up();
        assert_eq!(new_keypad(1), keypad);

        keypad = new_keypad(3);
        keypad.move_up();
        assert_eq!(new_keypad(3), keypad);

        keypad = new_keypad(8);
        keypad.move_up();
        assert_eq!(new_keypad(5), keypad);
    }

    #[test]
    fn test_move_down() {
        let mut keypad = new_keypad(7);
        keypad.move_down();
        assert_eq!(new_keypad(7), keypad);

        keypad = new_keypad(9);
        keypad.move_down();
        assert_eq!(new_keypad(9), keypad);

        keypad = new_keypad(2);
        keypad.move_down();
        assert_eq!(new_keypad(5), keypad);
    }
}

use crate::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NormalKeypad(u8);

impl NormalKeypad {
    fn move_left(&mut self) {
        match self.0 {
            1 | 4 | 7 => {}
            _ => self.0 -= 1,
        }
    }
    fn move_right(&mut self) {
        match self.0 {
            3 | 6 | 9 => {}
            _ => self.0 += 1,
        }
    }
    fn move_up(&mut self) {
        match self.0 {
            1..=3 => {}
            _ => self.0 -= 3,
        }
    }
    fn move_down(&mut self) {
        match self.0 {
            7..=9 => {}
            _ => self.0 += 3,
        }
    }
}

impl super::Keypad for NormalKeypad {
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
        10
    }
}

impl Default for NormalKeypad {
    fn default() -> Self {
        NormalKeypad(5)
    }
}
