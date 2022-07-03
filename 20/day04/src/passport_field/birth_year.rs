use derive_deref::Deref;

use super::*;

#[derive(Debug, Deref, PartialEq, Eq, Clone, Copy)]
pub struct BirthYear(Validity);

impl From<&str> for BirthYear {
    fn from(s: &str) -> Self {
        if let Ok(year) = s.parse::<usize>() {
            if s.len() == 4 && 1920 <= year && year <= 2002 {
                return Self(Valid);
            }
        }
        Self(Invalid)
    }
}

#[test]
fn test_birth_year() {
    assert_eq!(BirthYear::from("2002"), BirthYear(Valid));
    assert_eq!(BirthYear::from("2003"), BirthYear(Invalid));
}
