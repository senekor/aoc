use std::collections::{HashMap, HashSet};

pub mod graph;
mod parse;

type Name = String;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve {
    name: Name,
    flow_rate: usize,
    adjacent_valves: Vec<Name>,
}

fn rec(
    report: &HashMap<Name, Valve>,
    location: Name,
    opened_valves: &HashSet<Name>,
    minutes_remaining: usize,
) -> usize {
    let mut visited_valves = HashSet::new();
    visited_valves.insert(location.clone());
    // maps from name to distance and flow rate.
    // a choice is only plausible if it has a higher flow rate than any other
    // choice with a shorter distance.
    let mut plausible_next_choices = HashMap::new();

    let mut reachable_valves = report[&location]
        .adjacent_valves
        .iter()
        .map(|v| (v.clone(), (report[v].flow_rate, vec![v.clone()], 0)))
        .collect::<HashMap<_, _>>();

    for current_distance in 1..minutes_remaining {
        if visited_valves.len() == report.len() {
            break;
        }
        for (v, (flow_rate, path, mut path_max_flow_rate)) in std::mem::take(&mut reachable_valves)
        {
            visited_valves.insert(v.clone());

            // greater or equal not because it makes sense to go a further
            // distance for the same flow rate. but there might be some valves
            // with equal distance and equal flow rate, and we want to cosider
            // those.
            if !opened_valves.contains(&v) && flow_rate > path_max_flow_rate {
                plausible_next_choices.insert(v.clone(), (current_distance, flow_rate));
                path_max_flow_rate = flow_rate;
            }

            for vv in report[&v].adjacent_valves.clone() {
                if !visited_valves.contains(&vv) {
                    let mut path = path.clone();
                    path.push(vv.clone());
                    let flow_rate = report[&vv].flow_rate;
                    if let Some(existing) = reachable_valves.get_mut(&vv) {
                        if path_max_flow_rate > existing.2 {
                            *existing = (flow_rate, path, path_max_flow_rate);
                        }
                    } else {
                        reachable_valves.insert(vv, (flow_rate, path, path_max_flow_rate));
                    }
                }
            }
        }
    }

    plausible_next_choices
        .into_iter()
        .map(|(name, (distance, flow_rate))| {
            let mut opened_valves = opened_valves.clone();
            opened_valves.insert(name.clone());
            let new_minutes_remaining = minutes_remaining - distance - 1;
            let r = rec(report, name, &opened_valves, new_minutes_remaining);
            let a = flow_rate * new_minutes_remaining;
            r + a
        })
        .max()
        .unwrap_or_default()
}

pub fn part1(mut input: &str) -> usize {
    let report = parse::report(&mut input).unwrap();

    rec(&report, "AA".into(), &HashSet::new(), 30)
}

pub fn part2(mut input: &str) -> usize {
    #[allow(unused)]
    let valves = parse::report(&mut input).unwrap();
    0
}
