//! Provides a helper struct [Fail] to make the generated
//! tests nicer.

/// Use this to indicate a test result isn't available yet.
///
/// `PartialEq<Fail>` can be implemented for any type, as
/// it just returns false unconditionally. Add your type to
/// the macro invocation if you need to compare `Fail` to
/// something that's not covered yet.
#[derive(Debug)]
pub struct Fail;

macro_rules! impl_fail {
    ($($t:ty),+) => {
        $(
            impl PartialEq<Fail> for $t {
                fn eq(&self, _: &Fail) -> bool {
                    false
                }
            }
        )*
    };
}

impl_fail!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, String);
