use std::cmp::{max, min};
use std::collections::HashMap;

fn parse_line(line: &str) -> (&str, &str, i32) {
    let mut words = line.split(' ');
    let from = words.next().unwrap();
    words.next().unwrap();
    let to = words.next().unwrap();
    words.next().unwrap();
    let dist = words.next().unwrap().parse::<i32>().unwrap();
    (from, to, dist)
}

type Node<'a> = HashMap<&'a str, i32>;
type Graph<'a> = HashMap<&'a str, Node<'a>>;

fn constr_graph(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();
    for line in input.split('\n') {
        let (from, to, dist) = parse_line(line);
        if !graph.contains_key(from) {
            graph.insert(from, HashMap::new());
        }
        if !graph.contains_key(to) {
            graph.insert(to, HashMap::new());
        }
        graph.get_mut(from).unwrap().insert(to, dist);
        graph.get_mut(to).unwrap().insert(from, dist);
    }
    graph
}

fn except_start<'a>(locations: Vec<&'a str>, start: &str) -> Vec<&'a str> {
    locations
        .iter()
        .filter(|loc| **loc != start)
        .copied()
        .collect()
}

fn walk(graph: &Graph, start: &str, locs: Vec<&str>, use_max: bool) -> i32 {
    if locs.is_empty() {
        return 0;
    }

    let mut comp_dist = if use_max { i32::MIN } else { i32::MAX };
    for next in locs.clone() {
        let dist = graph.get(start).unwrap().get(next).unwrap();
        let rem_locs = except_start(locs.clone(), next);
        comp_dist = if use_max {
            max(comp_dist, dist + walk(graph, next, rem_locs, use_max))
        } else {
            min(comp_dist, dist + walk(graph, next, rem_locs, use_max))
        };
    }
    comp_dist
}

pub fn part1(input: &str) -> i32 {
    let graph = constr_graph(input);
    let locations: Vec<&str> = graph.keys().copied().collect();
    let mut min_dist = i32::MAX;
    for start in locations.clone() {
        let rem_locs = except_start(locations.clone(), start);
        min_dist = min(min_dist, walk(&graph, start, rem_locs, false))
    }
    min_dist
}

pub fn part2(input: &str) -> i32 {
    let graph = constr_graph(input);
    let locations: Vec<&str> = graph.keys().copied().collect();
    let mut max_dist = i32::MIN;
    for start in locations.clone() {
        let rem_locs = except_start(locations.clone(), start);
        max_dist = max(max_dist, walk(&graph, start, rem_locs, true))
    }
    max_dist
}
