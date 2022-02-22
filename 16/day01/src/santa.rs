enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

#[derive(Default)]
struct Location {
    x: i32,
    y: i32,
}

#[derive(Default)]
pub struct Santa {
    direction: Direction,
    location: Location,
}

use super::instruction::Instruction;
use super::instruction::Turn;

impl Santa {
    fn turn(&mut self, turn: Turn) {
        self.direction = match turn {
            Turn::Left => match self.direction {
                North => West,
                West => South,
                South => East,
                East => North,
            },
            Turn::Right => match self.direction {
                North => East,
                East => South,
                South => West,
                West => North,
            },
        }
    }

    fn move_forward(&mut self, distance: i32) {
        match self.direction {
            North => self.location.y += distance,
            East => self.location.x += distance,
            South => self.location.y -= distance,
            West => self.location.x -= distance,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        self.turn(instruction.get_turn());
        self.move_forward(instruction.get_distance());
    }

    pub fn get_distance(&self) -> i32 {
        self.location.x.abs() + self.location.y.abs()
    }
}
