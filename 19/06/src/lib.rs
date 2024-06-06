use std::collections::HashMap;

#[derive(Default)]
struct Object<'a> {
    parent: Option<&'a str>,
    children: Vec<&'a str>,
}

fn parse(input: &str) -> HashMap<&str, Object> {
    input.lines().map(|l| l.split_once(')').unwrap()).fold(
        HashMap::new(),
        |mut acc, (parent, child)| {
            acc.entry(parent).or_default().children.push(child);
            acc.entry(child).or_default().parent = Some(parent);
            acc
        },
    )
}

fn count_orbits(m: &HashMap<&str, Object>, name: &str, current_depth: usize) -> usize {
    let Some(body) = m.get(name) else {
        return 0;
    };
    let orbiting_bodies = &body.children;
    orbiting_bodies
        .iter()
        .map(|orbiting_body| count_orbits(m, orbiting_body, current_depth + 1))
        .sum::<usize>()
        + orbiting_bodies.len() * current_depth
}

pub fn part1(input: &str) -> usize {
    let m = parse(input);
    count_orbits(&m, "COM", 1)
}

pub fn part2(input: &str) -> usize {
    let m = parse(input);
    let you_path = find_path(&m, "YOU");
    let san_path = find_path(&m, "SAN");
    let num_duplicates = you_path
        .iter()
        .copied()
        .filter(|elem| san_path.contains(elem))
        .count();
    you_path.len() + san_path.len() - num_duplicates * 2 - 2
}

fn find_path<'a>(m: &HashMap<&'a str, Object<'a>>, name: &'a str) -> Vec<&'a str> {
    let Some(parent) = m.get(name).and_then(|obj| obj.parent) else {
        return vec![name];
    };
    let mut prev_path = find_path(m, parent);
    prev_path.push(name);
    prev_path
}
