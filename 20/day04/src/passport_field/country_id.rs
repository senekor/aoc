use derive_deref::Deref;

use super::*;

#[derive(Debug, Deref, PartialEq, Eq, Clone, Copy)]
pub struct CountryID(Validity);

impl From<&str> for CountryID {
    fn from(_: &str) -> Self {
        Self(Valid)
    }
}
