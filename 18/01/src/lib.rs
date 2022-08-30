use std::collections::HashSet;

pub fn part1(input: &str) -> i32 {
    input.lines().map(|line| line.parse::<i32>().unwrap()).sum()
}

pub fn part2(input: &str) -> i32 {
    let frequencies = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .cycle();
    let mut current = 0;
    let mut seen = HashSet::from([0]);
    for freq in frequencies {
        current += freq;
        if seen.contains(&current) {
            return current;
        }
        seen.insert(current);
    }
    panic!("")
}
