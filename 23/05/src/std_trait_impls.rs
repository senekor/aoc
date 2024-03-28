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

impl<T: Category> Clone for Range<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Category> Copy for Range<T> {}
impl<T: Category> PartialEq for Range<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.start, self.length) == (other.start, other.length)
    }
}
impl<T: Category> Eq for Range<T> {}
impl<T: Category> PartialOrd for Range<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Category> Ord for Range<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.start, self.length).cmp(&(other.start, other.length))
    }
}

// MapRange

impl<Src: Category, Dest: Category> Clone for MapRange<Src, Dest> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<Src: Category, Dest: Category> Copy for MapRange<Src, Dest> {}
impl<Src: Category, Dest: Category> PartialEq for MapRange<Src, Dest> {
    fn eq(&self, other: &Self) -> bool {
        (self.src, self.dest) == (other.src, other.dest)
    }
}
impl<Src: Category, Dest: Category> Eq for MapRange<Src, Dest> {}
impl<Src: Category, Dest: Category> PartialOrd for MapRange<Src, Dest> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<Src: Category, Dest: Category> Ord for MapRange<Src, Dest> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.src.cmp(&other.src)
    }
}

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
