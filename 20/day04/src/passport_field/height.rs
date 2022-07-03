use derive_deref::Deref;

use super::*;

#[derive(Debug, Deref, PartialEq, Eq, Clone, Copy)]
pub struct Height(Validity);

impl From<&str> for Height {
    fn from(s: &str) -> Self {
        if let Ok(height) = s[..s.len() - 2].parse::<usize>() {
            if (s.ends_with("cm") && 150 <= height && height <= 193)
                || (s.ends_with("in") && 59 <= height && height <= 76)
            {
                return Self(Valid);
            }
        }
        Self(Invalid)
    }
}

#[test]
fn test_height() {
    assert_eq!(Height::from("60in"), Height(Valid));
    assert_eq!(Height::from("190cm"), Height(Valid));
    assert_eq!(Height::from("190in"), Height(Invalid));
    assert_eq!(Height::from("190"), Height(Invalid));
}
