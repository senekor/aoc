use core::panic;

#[derive(Debug, Clone, Copy)]
pub enum Turn {
    Left,
    Right,
}

pub struct Instruction {
    turn: Turn,
    distance: i32,
}

impl std::str::FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let turn = match s.get(0..1).unwrap() {
            "R" => Turn::Right,
            "L" => Turn::Left,
            _ => panic!("unknown turn"),
        };
        let distance = s.get(1..).unwrap().parse().unwrap();

        Ok(Instruction { turn, distance })
    }
}

impl Instruction {
    pub fn get_turn(&self) -> Turn {
        self.turn
    }

    pub fn get_distance(&self) -> i32 {
        self.distance
    }
}
