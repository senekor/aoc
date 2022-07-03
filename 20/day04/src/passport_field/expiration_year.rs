use derive_deref::Deref;

use super::*;

#[derive(Debug, Deref, PartialEq, Eq, Clone, Copy)]
pub struct ExpirationYear(Validity);

impl From<&str> for ExpirationYear {
    fn from(s: &str) -> Self {
        if let Ok(year) = s.parse::<usize>() {
            if s.len() == 4 && 2020 <= year && year <= 2030 {
                return Self(Valid);
            }
        }
        Self(Invalid)
    }
}
