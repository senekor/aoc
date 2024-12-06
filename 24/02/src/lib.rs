use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|report| {
            let levels = report
                .split_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                .collect_vec();
            test(&levels)
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|report| {
            let levels = report
                .split_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                .collect_vec();
            if test(&levels) {
                return true;
            }
            for removal_candidate in 0..levels.len() {
                let mut levels = levels.clone();
                levels.remove(removal_candidate);
                if test(&levels) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn test(levels: &[i32]) -> bool {
    let ascending = levels[0] < levels[1];
    levels.iter().copied().tuple_windows().all(|(a, b)| {
        let correct_order = ascending == (a < b);
        let within_range = (1..=3).contains(&a.abs_diff(b));

        correct_order && within_range
    })
}
