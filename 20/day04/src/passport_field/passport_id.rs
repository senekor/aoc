use derive_deref::Deref;
use regex::Regex;

use super::*;

lazy_static::lazy_static! {
    static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

#[derive(Debug, Deref, PartialEq, Eq, Clone, Copy)]
pub struct PassportID(Validity);

impl From<&str> for PassportID {
    fn from(s: &str) -> Self {
        match PID_REGEX.is_match(s) {
            true => Self(Valid),
            _ => Self(Invalid),
        }
    }
}

#[test]
fn test_passport_id() {
    assert_eq!(PassportID::from("000000001"), PassportID(Valid));
    assert_eq!(PassportID::from("0123456789"), PassportID(Invalid));
}
