//! Provides a helper trait [Output] to make the generated
//! main output nicer.

/// Helper trait to provide nice main output for placholder
/// values.
pub trait Output: std::fmt::Display + Default + Eq {
    /// Provides nice output if the value is obviously a
    /// placeholder instead of an actual attempt to solve
    /// the puzzle. (empty string, 0, etc.)
    fn output(&self) -> String {
        if *self == Self::default() {
            "...is not yet implemented!".into()
        } else {
            format!("{self}")
        }
    }
}

macro_rules! impl_output {
    ($($t:ty),+) => {
        $(
            impl Output for $t {}
        )*
    };
}

impl_output!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &str, String);
