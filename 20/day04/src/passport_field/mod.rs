mod birth_year;
mod country_id;
mod expiration_year;
mod eye_color;
mod hair_color;
mod height;
mod issue_year;
mod passport_id;

pub use birth_year::BirthYear;
pub use country_id::CountryID;
pub use expiration_year::ExpirationYear;
pub use eye_color::EyeColor;
pub use hair_color::HairColor;
pub use height::Height;
pub use issue_year::IssueYear;
pub use passport_id::PassportID;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Validity {
    Valid,
    Invalid,
}
pub use Validity::*;

impl Validity {
    pub fn is_valid(&self) -> bool {
        *self == Valid
    }
}

pub trait OptionalField {
    fn exists_and_is_valid(&self) -> bool;
}

impl<T: std::ops::Deref<Target = Validity> + Copy> OptionalField for Option<T> {
    fn exists_and_is_valid(&self) -> bool {
        self.map(|v| v.is_valid()).unwrap_or(false)
    }
}
