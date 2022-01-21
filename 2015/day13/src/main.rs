use itertools::Itertools;
use std::collections::HashMap;

fn parse_line(line: &str) -> (&str, i32, &str) {
    let line = &line[..line.len() - 1]; // drop period
    let mut words = line.split(" ");
    let p1 = words.next().unwrap();
    let sign = words.nth(1).unwrap();
    let happy: i32 = words.next().unwrap().parse().unwrap();
    let p2 = words.nth(6).unwrap();
    if sign == "lose" {
        return (p1, -happy, p2);
    }
    (p1, happy, p2)
}

fn parse_table(table: &'static str) -> HashMap<&'static str, HashMap<&str, i32>> {
    let mut map: HashMap<&'static str, _> = HashMap::new();

    for line in table.lines() {
        let (p1, h, p2) = parse_line(line);
        let e = map.entry(p1).or_insert(HashMap::new());
        e.insert(p2, h);
    }

    map
}

fn part1(input: &'static str) {
    let map: HashMap<&'static str, _> = parse_table(input);
    let n = map.len();
    let mut max_happy = 0;
    for seating in map.iter().permutations(map.len()) {
        let mut happy = 0;
        happy += seating[0].1.get(seating[n - 1].0).unwrap();
        happy += seating[0].1.get(seating[1].0).unwrap();
        for i in 1..n - 1 {
            happy += seating[i].1.get(seating[i - 1].0).unwrap();
            happy += seating[i].1.get(seating[i + 1].0).unwrap();
        }
        happy += seating[n - 1].1.get(seating[n - 2].0).unwrap();
        happy += seating[n - 1].1.get(seating[0].0).unwrap();
        if happy > max_happy {
            max_happy = happy;
        }
    }
    println!("happiness of optimal seating arrangement: {}", max_happy)
}

fn part2(input: &'static str) {
    let mut map: HashMap<&'static str, _> = parse_table(input);
    let mut remos_prefs = HashMap::new();
    map.iter_mut().for_each(|(p, m)| {
        m.insert("remo", 0);
        remos_prefs.insert(*p, 0);
    });
    map.insert("remo", remos_prefs);
    let n = map.len();
    let mut max_happy = 0;
    for seating in map.iter().permutations(map.len()) {
        let mut happy = 0;
        happy += seating[0].1.get(seating[n - 1].0).unwrap();
        happy += seating[0].1.get(seating[1].0).unwrap();
        for i in 1..n - 1 {
            happy += seating[i].1.get(seating[i - 1].0).unwrap();
            happy += seating[i].1.get(seating[i + 1].0).unwrap();
        }
        happy += seating[n - 1].1.get(seating[n - 2].0).unwrap();
        happy += seating[n - 1].1.get(seating[0].0).unwrap();
        if happy > max_happy {
            max_happy = happy;
        }
    }
    println!("happiness of optimal seating arrangement: {}", max_happy)
}

fn main() {
    let input = include_str!("../input/input.txt");

    part1(input);

    part2(input);
}
