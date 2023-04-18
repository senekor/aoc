use std::collections::HashSet;

use itertools::Itertools;

struct Range {
    start: i64,
    end: i64,
}

struct Rule<'a> {
    name: &'a str,
    a: Range,
    b: Range,
}

fn parse_rule(rule: &str) -> Rule {
    let (name, ranges) = rule.split_once(": ").unwrap();
    let (a, b) = ranges.split_once(" or ").unwrap();
    let (a_start, a_end) = a.split_once('-').unwrap();
    let (b_start, b_end) = b.split_once('-').unwrap();
    Rule {
        name,
        a: Range {
            start: a_start.parse().unwrap(),
            end: a_end.parse().unwrap(),
        },
        b: Range {
            start: b_start.parse().unwrap(),
            end: b_end.parse().unwrap(),
        },
    }
}

fn prepare_input(input: &str) -> (Vec<Rule>, Vec<i64>, Vec<Vec<i64>>) {
    let mut iter = input.split("\n\n");
    let rules = iter.next().unwrap().lines().map(parse_rule).collect_vec();
    let my_ticket = iter
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect_vec();
    let tickets = iter
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect_vec())
        .collect_vec();
    (rules, my_ticket, tickets)
}

fn field_satisfies_rule(rule: &Rule, field: i64) -> bool {
    rule.a.start <= field && field <= rule.a.end || rule.b.start <= field && field <= rule.b.end
}

fn field_satisfies_some_rule(rules: &[Rule], field: i64) -> bool {
    for rule in rules {
        if field_satisfies_rule(rule, field) {
            return true;
        }
    }
    false
}

fn is_valid_ticket(rules: &[Rule], ticket: &[i64]) -> Result<(), i64> {
    let mut total_invalid = 0;
    let mut valid = true;
    for &field in ticket {
        if !field_satisfies_some_rule(rules, field) {
            valid = false;
            total_invalid += field;
        }
    }
    match valid {
        true => Ok(()),
        false => Err(total_invalid),
    }
}

pub fn part1(input: &str) -> i64 {
    let (rules, _my_ticket, tickets) = prepare_input(input);

    let mut error_rate = 0;
    for ticket in tickets {
        let is_valid_or_num_invalid = is_valid_ticket(&rules, &ticket);
        if let Err(num_invalid) = is_valid_or_num_invalid {
            error_rate += num_invalid
        }
    }
    error_rate
}

fn find_rule<'a>(rules: &'a [Rule], name: &str) -> Option<&'a Rule<'a>> {
    rules.iter().find(|r| r.name == name)
}

pub fn part2(input: &str) -> i64 {
    let (rules, my_ticket, tickets) = prepare_input(input);
    let n = my_ticket.len();
    let rule_names = rules.iter().map(|r| r.name).collect::<HashSet<_>>();
    let mut possible_names = vec![rule_names; n];
    for ticket in tickets {
        if is_valid_ticket(&rules, &ticket).is_err() {
            continue;
        }
        for (i, poss_names) in possible_names.iter_mut().enumerate() {
            poss_names
                .retain(|name| field_satisfies_rule(find_rule(&rules, name).unwrap(), ticket[i]));
        }
    }
    let mut all_len_one = false;
    while !all_len_one {
        for i in 0..n {
            if possible_names[i].len() == 1 {
                for j in 0..n {
                    if j != i {
                        possible_names[j] = possible_names[j]
                            .difference(&possible_names[i])
                            .copied()
                            .collect();
                    }
                }
            }
        }
        all_len_one = possible_names.iter().all(|names| names.len() == 1);
    }
    possible_names
        .into_iter()
        .zip(my_ticket.into_iter())
        .flat_map(|(names, ticket)| names.into_iter().map(move |name| (name, ticket)))
        .filter(|(name, _)| name.starts_with("departure"))
        .fold(1, |acc, (_, ticket)| acc * ticket)
}
