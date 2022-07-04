use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PassportField {
    Valid,
    Invalid,
    Missing,
}
pub use PassportField::*;

impl Default for PassportField {
    fn default() -> Self {
        Missing
    }
}

impl PassportField {
    pub fn exists(&self) -> bool {
        *self != Missing
    }
    pub fn is_valid(&self) -> bool {
        *self == Valid
    }
}

pub fn parse_birth_year(s: &str) -> PassportField {
    match s.parse::<usize>() {
        Ok(y) if s.len() == 4 && 1920 <= y && y <= 2002 => Valid,
        _ => Invalid,
    }
}

#[test]
fn test_birth_year() {
    assert_eq!(parse_birth_year("2002"), Valid);
    assert_eq!(parse_birth_year("2003"), Invalid);
}

pub fn parse_issue_year(s: &str) -> PassportField {
    match s.parse::<usize>() {
        Ok(y) if s.len() == 4 && 2010 <= y && y <= 2020 => Valid,
        _ => Invalid,
    }
}

pub fn parse_expiration_year(s: &str) -> PassportField {
    match s.parse::<usize>() {
        Ok(y) if s.len() == 4 && 2020 <= y && y <= 2030 => Valid,
        _ => Invalid,
    }
}

pub fn parse_height(s: &str) -> PassportField {
    if let Ok(h) = s.get(..s.len() - 2).unwrap_or("0").parse::<usize>() {
        if (s.ends_with("cm") && 150 <= h && h <= 193) || (s.ends_with("in") && 59 <= h && h <= 76)
        {
            return Valid;
        }
    }
    Invalid
}

#[test]
fn test_height() {
    assert_eq!(parse_height("60in"), Valid);
    assert_eq!(parse_height("190cm"), Valid);
    assert_eq!(parse_height("190in"), Invalid);
    assert_eq!(parse_height("190"), Invalid);
}

lazy_static::lazy_static! {
    static ref HC_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
}

pub fn parse_hair_color(s: &str) -> PassportField {
    match HC_REGEX.is_match(s) {
        true => Valid,
        _ => Invalid,
    }
}

#[test]
fn test_hair_color() {
    assert_eq!(parse_hair_color("#123abc"), Valid);
    assert_eq!(parse_hair_color("#123abz"), Invalid);
    assert_eq!(parse_hair_color("123abc"), Invalid);
}

pub fn parse_eye_color(s: &str) -> PassportField {
    match s {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Valid,
        _ => Invalid,
    }
}

#[test]
fn test_eye_color() {
    assert_eq!(parse_eye_color("brn"), Valid);
    assert_eq!(parse_eye_color("wat"), Invalid);
}

lazy_static::lazy_static! {
    static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

pub fn parse_passport_id(s: &str) -> PassportField {
    match PID_REGEX.is_match(s) {
        true => Valid,
        _ => Invalid,
    }
}

#[test]
fn test_passport_id() {
    assert_eq!(parse_passport_id("000000001"), Valid);
    assert_eq!(parse_passport_id("0123456789"), Invalid);
}

#[derive(Debug, Default)]
struct Passport([PassportField; 7]);

impl From<&str> for Passport {
    fn from(s: &str) -> Self {
        let mut res = Self::default();
        for field in s.split_whitespace() {
            match field.split_once(':').unwrap() {
                ("byr", v) => res.0[0] = parse_birth_year(v),
                ("iyr", v) => res.0[1] = parse_issue_year(v),
                ("eyr", v) => res.0[2] = parse_expiration_year(v),
                ("hgt", v) => res.0[3] = parse_height(v),
                ("hcl", v) => res.0[4] = parse_hair_color(v),
                ("ecl", v) => res.0[5] = parse_eye_color(v),
                ("pid", v) => res.0[6] = parse_passport_id(v),
                _ => (),
            }
        }
        res
    }
}

impl Passport {
    fn has_all_fields(&self) -> bool {
        self.0.iter().all(|pf| pf.exists())
    }

    fn is_valid(&self) -> bool {
        self.0.iter().all(|pf| pf.is_valid())
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
