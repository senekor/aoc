type Assignment = (u8, u8);

fn parse_assignment(s: &str) -> Assignment {
    let mut iter = s.split('-');
    (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    )
}

fn parse_assignment_pair(s: &str) -> (Assignment, Assignment) {
    let mut iter = s.split(',');
    (
        parse_assignment(iter.next().unwrap()),
        parse_assignment(iter.next().unwrap()),
    )
}

fn contains(a: Assignment, b: Assignment) -> bool {
    a.0 <= b.0 && b.1 <= a.1
}

fn overlap(a: Assignment, b: Assignment) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_assignment_pair)
        .filter(|p| contains(p.0, p.1) || contains(p.1, p.0))
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_assignment_pair)
        .filter(|p| overlap(p.0, p.1))
        .count()
}
