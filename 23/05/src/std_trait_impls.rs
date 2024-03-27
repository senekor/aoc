//! just a module to neatly stow away the boilerplate-y
//! implementations of standard library traits.
//!
//! cannot use derive because otherwise trait bounds on `Category`
//! marker generics would be required.
//!
//! it would be possible to add these as super traits to `Category`,
//! then it would be possible to derive them without trait bounds.
//! but that doesn't seem clean to me.

use super::*;

// Number

impl<T: Category> Clone for Number<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Category> Copy for Number<T> {}
impl<T: Category> PartialEq for Number<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
impl<T: Category> Eq for Number<T> {}
impl<T: Category> PartialOrd for Number<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Category> Ord for Number<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

// Range

impl<Src: Category, Dest: Category> Clone for Range<Src, Dest> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<Src: Category, Dest: Category> Copy for Range<Src, Dest> {}
impl<Src: Category, Dest: Category> PartialEq for Range<Src, Dest> {
    fn eq(&self, other: &Self) -> bool {
        self.destination_range_start == other.destination_range_start
            && self.source_range_start == other.source_range_start
            && self.range_length == other.range_length
    }
}
impl<Src: Category, Dest: Category> Eq for Range<Src, Dest> {}

// Map

impl<Src: Category, Dest: Category> Clone for Map<Src, Dest> {
    fn clone(&self) -> Self {
        Self {
            ranges: self.ranges.clone(),
        }
    }
}
impl<Src: Category, Dest: Category> PartialEq for Map<Src, Dest> {
    fn eq(&self, other: &Self) -> bool {
        self.ranges == other.ranges
    }
}
impl<Src: Category, Dest: Category> Eq for Map<Src, Dest> {}
