#[derive(Debug, Clone, Copy, Default)]
pub struct Row(u8);

impl std::ops::Shl<usize> for Row {
    type Output = Option<Self>;

    fn shl(self, rhs: usize) -> Self::Output {
        if (self.0 & 0b_0100_0000) != 0 {
            return None;
        }
        Some(Self(self.0 << rhs))
    }
}

impl std::ops::Shr<usize> for Row {
    type Output = Option<Self>;

    fn shr(self, rhs: usize) -> Self::Output {
        if (self.0 & 1) != 0 {
            return None;
        }
        Some(Self(self.0 >> rhs))
    }
}

impl std::ops::BitAnd for Row {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Row {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Row {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        value
            .chars()
            .chain(std::iter::once('.').cycle())
            .take(5)
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("unexpected character '{c}' while parsing row"),
            })
            .fold(Row(0), |acc, next| (acc << 1).unwrap() | Row(next))
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..7).rev() {
            let c = match (self.0 >> i) & 1 {
                0 => '.',
                1 => '#',
                _ => unreachable!(),
            };
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

impl Row {
    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        !(*self & *other).is_empty()
    }
}
