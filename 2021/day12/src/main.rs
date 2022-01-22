use itertools::*;
use std::collections::{HashMap, HashSet};

type Line = (String, String);
fn parse(line: &str) -> Line {
    {
        let mut iter = line.split('-');
        (
            iter.next().unwrap().parse::<String>().unwrap(),
            iter.next().unwrap().parse::<String>().unwrap(),
        )
    }
}

#[derive(Default, Debug)]
struct Cave {
    neighbors: HashSet<String>,
}

fn traverse(
    caves: &HashMap<String, Cave>,
    small_visited: &mut HashSet<String>,
    current_cave: String,
) -> usize {
    let mut sum = 0;
    for n in caves.get(&current_cave).unwrap().neighbors.iter() {
        if n == "end" {
            sum += 1;
            continue;
        }
        if n.chars().next().unwrap().is_lowercase() {
            if small_visited.contains(n) {
                continue;
            }
            small_visited.insert(n.clone());
        }
        sum += traverse(caves, small_visited, n.to_string());
        if small_visited.contains(n) {
            small_visited.remove(n);
        }
    }
    sum
}

fn part1(input: Vec<Line>) {
    let mut caves: HashMap<String, Cave> = HashMap::new();
    for (from, to) in input {
        caves
            .entry(from.clone())
            .or_default()
            .neighbors
            .insert(to.clone());
        caves.entry(to).or_default().neighbors.insert(from);
    }

    let mut small_visited: HashSet<String> = HashSet::new();
    small_visited.insert("start".to_string());
    let num_routes = traverse(&caves, &mut small_visited, "start".to_string());

    println!("{:?}", num_routes)
}

fn traverse_2(
    caves: &HashMap<String, Cave>,
    small_visited: &mut HashSet<String>,
    current_cave: String,
    joker: bool,
) -> usize {
    let mut sum = 0;
    for n in caves.get(&current_cave).unwrap().neighbors.iter() {
        if n == "end" {
            sum += 1;
            continue;
        } else if n == "start" {
            continue;
        }
        let mut set_joker = false;
        let mut joker = joker;
        if n.chars().next().unwrap().is_lowercase() {
            if small_visited.contains(n) {
                if joker {
                    joker = false;
                    set_joker = true;
                } else {
                    continue;
                }
            } else {
                small_visited.insert(n.to_string());
            }
        }
        sum += traverse_2(caves, small_visited, n.to_string(), joker);
        if small_visited.contains(n) && !set_joker {
            small_visited.remove(n);
        }
    }
    sum
}

fn part2(input: Vec<Line>) {
    let mut caves: HashMap<String, Cave> = HashMap::new();
    for (from, to) in input {
        caves
            .entry(from.clone())
            .or_default()
            .neighbors
            .insert(to.clone());
        caves.entry(to).or_default().neighbors.insert(from);
    }

    let joker = true;
    let mut small_visited: HashSet<String> = HashSet::new();
    small_visited.insert("start".to_string());
    let num_routes = traverse_2(&caves, &mut small_visited, "start".to_string(), joker);

    println!("{:?}", num_routes)
}

fn main() {
    let input = include_str!("../input/input.txt");
    let input = input.lines().map(parse).collect_vec();

    part1(input.clone());

    part2(input);
}
