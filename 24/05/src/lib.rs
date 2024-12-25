use std::collections::HashMap;

pub fn part1(input: &str) -> i32 {
    let (page_ordering_rules, updates) = input.split_once("\n\n").unwrap();

    let page_ordering_rules = {
        let mut hm = HashMap::<_, Vec<_>>::new();
        for (before, after) in page_ordering_rules
            .lines()
            .filter_map(|l| l.split_once('|'))
        {
            hm.entry(before).or_default().push(after);
        }
        hm
    };
    let updates = updates
        .lines()
        .map(|l| l.split(',').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    'updates_loop: for update in updates {
        'pages_loop: for i in 0..update.len() {
            let page = update[i];
            let Some(deps) = page_ordering_rules.get(page) else {
                // page has no deps, no need to check anything
                continue 'pages_loop;
            };
            for prev_page in &update[0..i] {
                if deps.contains(prev_page) {
                    // update is invalid
                    continue 'updates_loop;
                }
            }
        }
        // update is valid
        sum += update[update.len() / 2].parse::<i32>().unwrap();
    }
    sum
}

pub fn part2(input: &str) -> usize {
    0
}
