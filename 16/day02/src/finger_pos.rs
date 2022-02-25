#[cfg(test)]
mod tests {
    use super::FingerPosition;

    fn new_finger_pos(val: u8) -> FingerPosition {
        FingerPosition(val)
    }

    #[test]
    fn test_move_left() {
        let mut finger_pos = new_finger_pos(1);
        finger_pos.move_left();
        assert_eq!(new_finger_pos(1), finger_pos);

        finger_pos = new_finger_pos(7);
        finger_pos.move_left();
        assert_eq!(new_finger_pos(7), finger_pos);

        finger_pos = new_finger_pos(6);
        finger_pos.move_left();
        assert_eq!(new_finger_pos(5), finger_pos);
    }

    #[test]
    fn test_move_right() {
        let mut finger_pos = new_finger_pos(3);
        finger_pos.move_right();
        assert_eq!(new_finger_pos(3), finger_pos);

        finger_pos = new_finger_pos(9);
        finger_pos.move_right();
        assert_eq!(new_finger_pos(9), finger_pos);

        finger_pos = new_finger_pos(4);
        finger_pos.move_right();
        assert_eq!(new_finger_pos(5), finger_pos);
    }

    #[test]
    fn test_move_up() {
        let mut finger_pos = new_finger_pos(1);
        finger_pos.move_up();
        assert_eq!(new_finger_pos(1), finger_pos);

        finger_pos = new_finger_pos(3);
        finger_pos.move_up();
        assert_eq!(new_finger_pos(3), finger_pos);

        finger_pos = new_finger_pos(8);
        finger_pos.move_up();
        assert_eq!(new_finger_pos(5), finger_pos);
    }

    #[test]
    fn test_move_down() {
        let mut finger_pos = new_finger_pos(7);
        finger_pos.move_down();
        assert_eq!(new_finger_pos(7), finger_pos);

        finger_pos = new_finger_pos(9);
        finger_pos.move_down();
        assert_eq!(new_finger_pos(9), finger_pos);

        finger_pos = new_finger_pos(2);
        finger_pos.move_down();
        assert_eq!(new_finger_pos(5), finger_pos);
    }
}

use super::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FingerPosition(u8);

impl FingerPosition {
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

    pub fn change_pos(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
        }
    }

    pub fn get_pos(&self) -> u8 {
        self.0
    }
}

impl Default for FingerPosition {
    fn default() -> Self {
        FingerPosition(5)
    }
}
