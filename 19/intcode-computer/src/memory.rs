use std::ops::{Index, IndexMut};

/// Encapsulates the behavior of the Intcode computer's memory.
#[derive(Debug, Clone)]
pub struct Memory(Vec<i64>);

impl From<&str> for Memory {
    fn from(s: &str) -> Self {
        Self(s.trim().split(',').map(|x| x.parse().unwrap()).collect())
    }
}

impl Index<usize> for Memory {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.get(index).unwrap_or(&0)
    }
}
impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.0.len() <= index {
            self.0.resize(index + 1, 0)
        }
        self.0.get_mut(index).unwrap()
    }
}

impl Memory {
    pub fn iter(&self) -> std::slice::Iter<i64> {
        self.0.iter()
    }
}
