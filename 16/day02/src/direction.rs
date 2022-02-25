pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

impl std::str::FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            "D" => Ok(Down),
            _ => panic!("unexpected direction: '{}'", s),
        }
    }
}
