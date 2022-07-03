use derive_deref::Deref;
use regex::Regex;

use super::*;

#[derive(Debug, Deref, PartialEq, Eq, Clone, Copy)]
pub struct HairColor(Validity);

lazy_static::lazy_static! {
    static ref HC_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
}

impl From<&str> for HairColor {
    fn from(s: &str) -> Self {
        match HC_REGEX.is_match(s) {
            true => Self(Valid),
            _ => Self(Invalid),
        }
    }
}

#[test]
fn test_hair_color() {
    assert_eq!(HairColor::from("#123abc"), HairColor(Valid));
    assert_eq!(HairColor::from("#123abz"), HairColor(Invalid));
    assert_eq!(HairColor::from("123abc"), HairColor(Invalid));
}
