use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Bag<'a>(&'a str);

#[derive(Debug)]
struct BagContent<'a>(Vec<(Bag<'a>, usize)>);

impl<'a> From<&'a str> for BagContent<'a> {
    fn from(s: &'a str) -> Self {
        if s == "no other bags." {
            return BagContent(vec![]);
        }
        BagContent(
            s.split(", ")
                .map(|num_and_bag| {
                    let (num, rest) = num_and_bag.split_once(' ').unwrap();
                    let (bag, _) = rest.split_once(" bag").unwrap();
                    (Bag(bag), num.parse().unwrap())
                })
                .collect(),
        )
    }
}

#[derive(Debug)]
struct BagRules<'a>(HashMap<Bag<'a>, BagContent<'a>>);

impl<'a> From<&'a str> for BagRules<'a> {
    fn from(s: &'a str) -> Self {
        BagRules(
            s.lines()
                .map(|line| {
                    let (bag, content) = line.split_once(" bags contain ").unwrap();
                    (Bag(bag), content.into())
                })
                .collect(),
        )
    }
}

pub fn part1(input: &str) -> usize {
    let bag_rules = BagRules::from(input);

    let mut reachable = HashSet::new();
    reachable.insert(Bag("shiny gold"));

    'outer: loop {
        for (bag, content) in bag_rules.0.iter() {
            if !reachable.contains(bag)
                && content
                    .0
                    .iter()
                    .any(|(content, _)| reachable.contains(content))
            {
                reachable.insert(*bag);
                continue 'outer;
            }
        }
        break;
    }

    reachable.len() - 1
}

impl<'a> Bag<'a> {
    fn get_content_count(
        &'a self,
        rules: &'a BagRules<'a>,
        cache: &mut HashMap<Bag<'a>, usize>,
    ) -> usize {
        if let Some(&res) = cache.get(self) {
            return res;
        }
        let res = rules
            .0
            .get(self)
            .unwrap()
            .0
            .iter()
            .map(|(bag, count)| (bag.get_content_count(rules, cache) + 1) * *count)
            .sum();
        cache.insert(*self, res);
        res
    }
}

pub fn part2(input: &str) -> usize {
    let rules = BagRules::from(input);
    let mut cache = HashMap::new();
    Bag("shiny gold").get_content_count(&rules, &mut cache)
}
