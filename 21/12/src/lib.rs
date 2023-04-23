use std::collections::{HashMap, HashSet};

use utils::Itertools;

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

pub fn part1(input: &str) -> usize {
    let input = input.lines().map(parse).collect_vec();

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
    traverse(&caves, &mut small_visited, "start".to_string())
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
        let mut mut_joker = joker;
        if n.chars().next().unwrap().is_lowercase() {
            if small_visited.contains(n) {
                if mut_joker {
                    mut_joker = false;
                    set_joker = true;
                } else {
                    continue;
                }
            } else {
                small_visited.insert(n.to_string());
            }
        }
        sum += traverse_2(caves, small_visited, n.to_string(), mut_joker);
        if small_visited.contains(n) && !set_joker {
            small_visited.remove(n);
        }
    }
    sum
}

pub fn part2(input: &str) -> usize {
    let input = input.lines().map(parse).collect_vec();

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
    traverse_2(&caves, &mut small_visited, "start".to_string(), joker)
}
