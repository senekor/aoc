use std::collections::HashMap;

use utils::Itertools;

pub fn part1(input: &str) -> u32 {
    let mut doubles = 0;
    let mut triples = 0;
    for line in input.lines() {
        let mut counts: HashMap<char, u32> = HashMap::new();
        for c in line.chars() {
            *counts.entry(c).or_default() += 1;
        }
        for (_, count) in counts.iter() {
            if *count == 2 {
                doubles += 1;
                break;
            }
        }
        for (_, count) in counts {
            if count == 3 {
                triples += 1;
                break;
            }
        }
    }
    doubles * triples
}

trait HemmingTools {
    fn hemming_dist(&self, other: &Self) -> usize;
    fn hemming_intersection(&self, other: &Self) -> String;
}

impl HemmingTools for str {
    fn hemming_dist(&self, other: &str) -> usize {
        self.chars()
            .zip(other.chars())
            .filter(|(l, r)| l != r)
            .count()
    }
    fn hemming_intersection(&self, other: &str) -> String {
        self.chars()
            .zip(other.chars())
            .filter_map(|(l, r)| if l == r { Some(l) } else { None })
            .collect::<String>()
    }
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .cartesian_product(input.lines())
        .filter_map(|(l, r)| {
            if l.hemming_dist(r) == 1 {
                Some(l.hemming_intersection(r))
            } else {
                None
            }
        })
        .next()
        .unwrap()
}
