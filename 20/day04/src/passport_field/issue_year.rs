use derive_deref::Deref;

use super::*;

#[derive(Debug, Deref, PartialEq, Eq, Clone, Copy)]
pub struct IssueYear(Validity);

impl From<&str> for IssueYear {
    fn from(s: &str) -> Self {
        if let Ok(year) = s.parse::<usize>() {
            if s.len() == 4 && 2010 <= year && year <= 2020 {
                return Self(Valid);
            }
        }
        Self(Invalid)
    }
}
