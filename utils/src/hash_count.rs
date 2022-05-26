//! Quickly count the number of occurrences of each
//! distinct item in an iterator. Use by calling `.collect::<HashCount<_>>()`
//! on the iterator who's items you want to count.

use std::{collections::HashMap, hash::Hash};

/// Wrapper for HashMap. Internal HashMap is intentionally
/// exposed, to ensure maximum interoperability.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HashCount<T: Eq + Hash>(pub HashMap<T, usize>);

impl<T: Eq + Hash> Default for HashCount<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Eq + Hash> Extend<T> for HashCount<T> {
    fn extend<U: IntoIterator<Item = T>>(&mut self, iter: U) {
        for elem in iter {
            *self.0.entry(elem).or_default() += 1;
        }
    }
}

impl<T: Eq + Hash> FromIterator<T> for HashCount<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut count = Self::default();
        count.extend(iter);
        count
    }
}

#[test]
fn sanity_check() {
    let count = "aab".chars().collect::<HashCount<char>>();
    assert_eq!(count.0.len(), 2);
    assert_eq!(count.0.get(&'a').unwrap(), &2);
    assert_eq!(count.0.get(&'b').unwrap(), &1);
}
