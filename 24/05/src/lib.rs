use std::collections::HashMap;

fn parse(input: &str) -> (HashMap<&str, Vec<&str>>, Vec<Vec<&str>>) {
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

    (page_ordering_rules, updates)
}

pub fn part1(input: &str) -> i32 {
    let (page_ordering_rules, updates) = parse(input);

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

pub fn part2(input: &str) -> i32 {
    let (page_ordering_rules, updates) = parse(input);

    let mut sum = 0;
    for mut update in updates {
        let mut was_incorrect = false;
        let mut i = 0;
        'pages_loop: while i < update.len() {
            let page = update[i];
            let Some(deps) = page_ordering_rules.get(page) else {
                // page has no deps, no need to check anything
                i += 1;
                continue 'pages_loop;
            };
            for k in 0..i {
                #[allow(clippy::borrow_deref_ref, reason = "won't compile otherwise")]
                let prev_page = &&*update[k];
                if deps.contains(prev_page) {
                    was_incorrect = true;
                    // fix update
                    update.remove(k);
                    update.insert(i, prev_page);
                    // between k..i may be dependencies of prev_page so we need
                    // to run the check loop for the same index i again.
                    continue 'pages_loop;
                }
            }
            i += 1;
        }
        if was_incorrect {
            sum += update[update.len() / 2].parse::<i32>().unwrap();
        }
    }
    sum
}
