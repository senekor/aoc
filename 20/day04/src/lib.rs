mod passport_field;
use passport_field::{
    BirthYear, CountryID, ExpirationYear, EyeColor, HairColor, Height, IssueYear, OptionalField,
    PassportID,
};

#[derive(Debug, Default)]
struct Passport {
    birth_year: Option<BirthYear>,
    issue_year: Option<IssueYear>,
    expiration_year: Option<ExpirationYear>,
    height: Option<Height>,
    hair_color: Option<HairColor>,
    eye_color: Option<EyeColor>,
    passpor_id: Option<PassportID>,
    coutry_id: Option<CountryID>,
}

impl From<&str> for Passport {
    fn from(s: &str) -> Self {
        let mut res = Self::default();
        for field in s.split_whitespace() {
            let (name, value) = field.split_once(':').unwrap();
            match name {
                "byr" => res.birth_year = Some(BirthYear::from(value)),
                "iyr" => res.issue_year = Some(IssueYear::from(value)),
                "eyr" => res.expiration_year = Some(ExpirationYear::from(value)),
                "hgt" => res.height = Some(Height::from(value)),
                "hcl" => res.hair_color = Some(HairColor::from(value)),
                "ecl" => res.eye_color = Some(EyeColor::from(value)),
                "pid" => res.passpor_id = Some(PassportID::from(value)),
                "cid" => res.coutry_id = Some(CountryID::from(value)),
                _ => panic!("unknown field name '{name}'"),
            }
        }
        res
    }
}

impl Passport {
    fn has_all_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passpor_id.is_some()
    }

    fn is_valid(&self) -> bool {
        self.birth_year.exists_and_is_valid()
            && self.issue_year.exists_and_is_valid()
            && self.expiration_year.exists_and_is_valid()
            && self.height.exists_and_is_valid()
            && self.hair_color.exists_and_is_valid()
            && self.eye_color.exists_and_is_valid()
            && self.passpor_id.exists_and_is_valid()
    }
}

struct Batch(Vec<Passport>);

impl From<&str> for Batch {
    fn from(s: &str) -> Self {
        Batch(s.split("\n\n").map(|p| p.into()).collect())
    }
}

impl Batch {
    fn count_complete_passports(&self) -> usize {
        self.0.iter().filter(|p| p.has_all_fields()).count()
    }

    fn count_valid_passports(&self) -> usize {
        self.0.iter().filter(|p| p.is_valid()).count()
    }
}

pub fn part1(input: &str) -> usize {
    Batch::from(input).count_complete_passports()
}

pub fn part2(input: &str) -> usize {
    Batch::from(input).count_valid_passports()
}
