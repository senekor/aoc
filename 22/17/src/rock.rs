use arrayvec::ArrayVec;

use crate::{jet::Jet, row::Row};

#[derive(Debug, Clone, Default)]
pub struct Rock(ArrayVec<Row, 4>);

impl From<&str> for Rock {
    fn from(value: &str) -> Self {
        let mut res = Self::default();
        for row in value.lines().map(Row::from) {
            res.0.push(row);
        }
        res.0.reverse();
        res
    }
}

#[rustfmt::skip]
static ROCKS: &[&str] = &[
"####",

"\
.#.
###
.#.",

"\
..#
..#
###",

"\
#
#
#
#",

"\
##
##",
];

pub fn get_falling_rocks() -> impl Iterator<Item = Rock> {
    ROCKS.iter().copied().map(Rock::from).cycle()
}

impl std::ops::Shl<usize> for Rock {
    type Output = Option<Self>;

    fn shl(mut self, rhs: usize) -> Self::Output {
        for row in self.0.iter_mut() {
            *row = (*row << rhs)?;
        }
        Some(self)
    }
}

impl std::ops::Shr<usize> for Rock {
    type Output = Option<Self>;

    fn shr(mut self, rhs: usize) -> Self::Output {
        for row in self.0.iter_mut() {
            *row = (*row >> rhs)?;
        }
        Some(self)
    }
}

impl Rock {
    pub fn collides_with(&self, other: &[Row], offset: usize) -> bool {
        if other.len() <= offset {
            return false;
        }
        self.0
            .iter()
            .zip(other[offset..].iter())
            .any(|(a, b)| a.collides_with(b))
    }

    pub fn apply(&self, jet: Jet) -> Option<Self> {
        match jet {
            Jet::Left => self.clone() << 1,
            Jet::Right => self.clone() >> 1,
        }
    }

    pub fn place_into(self, room: &mut Vec<Row>, offset: usize) {
        room.resize(room.len().max(offset + self.0.len()), Row::default());
        for (i, row) in self.0.into_iter().enumerate() {
            room[i + offset] |= row
        }
    }
}
