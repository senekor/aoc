use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    let mut res = String::new();
    let lines: Vec<_> = input.lines().collect();
    for i in 0..lines[0].len() {
        let mut counter: HashMap<char, usize> = HashMap::new();
        for line in lines.iter() {
            *counter.entry(line.chars().nth(i).unwrap()).or_default() += 1;
        }
        let max = counter
            .into_iter()
            .reduce(|acc, item| if acc.1 < item.1 { item } else { acc })
            .unwrap()
            .0;
        res.push(max)
    }
    res
}

pub fn part2(input: &str) -> String {
    let mut res = String::new();
    let lines: Vec<_> = input.lines().collect();
    for i in 0..lines[0].len() {
        let mut counter: HashMap<char, usize> = HashMap::new();
        for line in lines.iter() {
            *counter.entry(line.chars().nth(i).unwrap()).or_default() += 1;
        }
        let max = counter
            .into_iter()
            .reduce(|acc, item| if acc.1 > item.1 { item } else { acc })
            .unwrap()
            .0;
        res.push(max)
    }
    res
}
