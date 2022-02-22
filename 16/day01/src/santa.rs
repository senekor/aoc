enum Direction {
    North,
    East,
    South,
    West,
}
use std::collections::HashSet;

use Direction::*;

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

#[derive(Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Location {
    x: i32,
    y: i32,
}

#[derive(Default)]
pub struct Santa {
    direction: Direction,
    location: Location,
    visited: HashSet<Location>,
    first_visited_twice: Option<Location>,
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

    fn step_forward(&mut self) {
        match self.direction {
            North => self.location.y += 1,
            East => self.location.x += 1,
            South => self.location.y -= 1,
            West => self.location.x -= 1,
        }
    }

    fn move_forward(&mut self, distance: u32) {
        for _ in 0..distance {
            self.step_forward();
            if self.first_visited_twice.is_none() && !self.visited.insert(self.location) {
                self.first_visited_twice = Some(self.location);
            }
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        self.turn(instruction.get_turn());
        self.move_forward(instruction.get_distance());
    }

    pub fn get_distance(&self) -> i32 {
        self.location.x.abs() + self.location.y.abs()
    }

    pub fn get_first_visited_twice_distance(&self) -> i32 {
        let loc = self
            .first_visited_twice
            .expect("didn't find a location visited twice");

        loc.x.abs() + loc.y.abs()
    }
}
