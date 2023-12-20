#[derive(Debug, Clone, Copy)]
pub enum Jet {
    Left,
    Right,
}

impl From<char> for Jet {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("unexpected character {c} while parsing jet"),
        }
    }
}
