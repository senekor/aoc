use std::collections::HashMap;

struct Program {
    name: String,
    weight: u32,
    holds: Vec<String>,
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        let mut iter = value
            .split([' ', '(', ')', '-', '>', ','])
            .filter(|s| !s.is_empty());
        Self {
            name: iter.next().unwrap().into(),
            weight: iter.next().unwrap().parse().unwrap(),
            holds: iter.map(String::from).collect(),
        }
    }
}

pub fn part1(input: &str) -> String {
    let programs = input.lines().map(Program::from).collect::<Vec<_>>();
    programs
        .iter()
        .map(|p| &p.name)
        .find(|name| programs.iter().all(|other| !other.holds.contains(name)))
        .unwrap()
        .into()
}

// fails upon finding imbalance
fn total_weight(name: &str, programs: &HashMap<String, Program>) -> Result<u32, u32> {
    let children: Vec<_> = programs[name]
        .holds
        .iter()
        .map(|n| total_weight(n, programs).map(|w| (n, w)))
        .collect::<Result<_, _>>()?;

    if children.windows(2).all(|w| w[0].1 == w[1].1) {
        return Ok(programs[name].weight + children.into_iter().map(|c| c.1).sum::<u32>());
    }

    let w = children
        .windows(3)
        .find(|w| w[0].1 != w[1].1 || w[1].1 != w[2].1)
        .unwrap();
    let (correct, wrong) = match w {
        _ if w[0].1 == w[1].1 => (w[0].1, w[2]),
        _ if w[0].1 == w[2].1 => (w[0].1, w[1]),
        _ if w[1].1 == w[2].1 => (w[1].1, w[0]),
        _ => panic!(),
    };
    Err(programs[wrong.0.as_str()].weight + correct - wrong.1)
}

pub fn part2(input: &str) -> u32 {
    let programs: HashMap<_, _> = input
        .lines()
        .map(|s| {
            let p = Program::from(s);
            (p.name.clone(), p)
        })
        .collect();
    total_weight(&part1(input), &programs).unwrap_err()
}
