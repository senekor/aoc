use derive_deref::Deref;

use super::*;

#[derive(Debug, Deref, PartialEq, Eq, Clone, Copy)]
pub struct EyeColor(Validity);

impl From<&str> for EyeColor {
    fn from(s: &str) -> Self {
        match s {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Self(Valid),
            _ => Self(Invalid),
        }
    }
}

#[test]
fn test_eye_color() {
    assert_eq!(EyeColor::from("brn"), EyeColor(Valid));
    assert_eq!(EyeColor::from("wat"), EyeColor(Invalid));
}
