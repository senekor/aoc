use std::str::FromStr;

#[derive(Debug, PartialEq, Default)]
struct Sue {
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}

impl FromStr for Sue {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Sue, Self::Err> {
        let mut tokens = line.split(&[':', ','][..]);
        tokens.next(); // consume intro
        let mut sue = Sue::default();
        while let Some(property) = tokens.next() {
            let value: i32 = tokens.next().unwrap().trim().parse().unwrap();
            match property.trim() {
                "children" => sue.children = Some(value),
                "cats" => sue.cats = Some(value),
                "samoyeds" => sue.samoyeds = Some(value),
                "pomeranians" => sue.pomeranians = Some(value),
                "akitas" => sue.akitas = Some(value),
                "vizslas" => sue.vizslas = Some(value),
                "goldfish" => sue.goldfish = Some(value),
                "trees" => sue.trees = Some(value),
                "cars" => sue.cars = Some(value),
                "perfumes" => sue.perfumes = Some(value),
                p => panic!("unknown property '{}'", p),
            }
        }
        Ok(sue)
    }
}

#[test]
fn test_parse_sue() {
    let sue: Sue = ("Sue 1:".to_string()
        + "children: 1,"
        + "cats: 1,"
        + "samoyeds: 1,"
        + "pomeranians: 1,"
        + "akitas: 1,"
        + "vizslas: 1,"
        + "goldfish: 1,"
        + "trees: 1,"
        + "cars: 1,"
        + "perfumes: 1")
        .parse()
        .unwrap();
    assert_eq!(
        sue,
        Sue {
            children: Some(1),
            cats: Some(1),
            samoyeds: Some(1),
            pomeranians: Some(1),
            akitas: Some(1),
            vizslas: Some(1),
            goldfish: Some(1),
            trees: Some(1),
            cars: Some(1),
            perfumes: Some(1),
        }
    )
}

fn parse_sues(input: &str) -> Vec<Sue> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

impl Sue {
    fn matches(&self, other: &Sue) -> bool {
        (self.children == other.children || other.children.is_none())
            && (self.cats == other.cats || other.cats.is_none())
            && (self.samoyeds == other.samoyeds || other.samoyeds.is_none())
            && (self.pomeranians == other.pomeranians || other.pomeranians.is_none())
            && (self.akitas == other.akitas || other.akitas.is_none())
            && (self.vizslas == other.vizslas || other.vizslas.is_none())
            && (self.goldfish == other.goldfish || other.goldfish.is_none())
            && (self.trees == other.trees || other.trees.is_none())
            && (self.cars == other.cars || other.cars.is_none())
            && (self.perfumes == other.perfumes || other.perfumes.is_none())
    }
    fn matches2(&self, other: &Sue) -> bool {
        (self.children == other.children || other.children.is_none())
            && (self.cats < other.cats || other.cats.is_none())
            && (self.samoyeds == other.samoyeds || other.samoyeds.is_none())
            && (self.pomeranians > other.pomeranians || other.pomeranians.is_none())
            && (self.akitas == other.akitas || other.akitas.is_none())
            && (self.vizslas == other.vizslas || other.vizslas.is_none())
            && (self.goldfish > other.goldfish || other.goldfish.is_none())
            && (self.trees < other.trees || other.trees.is_none())
            && (self.cars == other.cars || other.cars.is_none())
            && (self.perfumes == other.perfumes || other.perfumes.is_none())
    }
}

const KNOWN_SUE: Sue = Sue {
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

pub fn part1(input: &str) -> usize {
    let sues = parse_sues(input);

    for (i, sue) in sues.iter().enumerate() {
        if KNOWN_SUE.matches(sue) {
            return i + 1;
        }
    }
    panic!()
}

pub fn part2(input: &str) -> usize {
    let sues = parse_sues(input);

    for (i, sue) in sues.iter().enumerate() {
        if KNOWN_SUE.matches2(sue) {
            return i + 1;
        }
    }
    panic!()
}
