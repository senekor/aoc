use std::collections::{HashMap, HashSet, VecDeque};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Actor {
    eta: usize,
    goal: Name,
}

fn rec2(
    report: &HashMap<Name, Valve>,
    distance_map: &HashMap<(Name, Name), usize>,
    closed_valves: HashSet<Name>,
    mut minutes_remaining: usize,
    mut actors: [Actor; 2],
) -> usize {
    actors.sort();
    if actors[0].eta != 0 {
        let wait = actors[0].eta;
        if wait > minutes_remaining {
            return 0;
        }
        actors[0].eta = 0;
        actors[1].eta -= wait;
        minutes_remaining -= wait;
    }

    let mut res_max = 0;
    for valve in closed_valves.iter() {
        let eta = distance_map[&(actors[0].goal.clone(), valve.clone())] + 1;
        let new_actor = Actor {
            eta,
            goal: valve.clone(),
        };
        let pressure_relief = minutes_remaining.saturating_sub(eta) * report[valve].flow_rate;
        let mut closed_valves = closed_valves.clone();
        closed_valves.remove(valve);
        let rec_res = rec2(
            report,
            distance_map,
            closed_valves,
            minutes_remaining,
            [new_actor, actors[1].clone()],
        );
        let rec_res = rec_res + pressure_relief;
        if rec_res > res_max {
            res_max = rec_res;
        }
    }
    res_max
}

pub fn part2(mut input: &str) -> usize {
    let report = parse::report(&mut input).unwrap();

    let distance_map = get_distance_map(&report);

    let closed_valves = report
        .iter()
        // consider zero flow rate valves as already closed for efficiency
        .filter_map(|(k, v)| (v.flow_rate != 0).then_some(k))
        .cloned()
        .collect();

    let me = Actor {
        eta: 0,
        goal: "AA".into(),
    };
    let elephant = Actor {
        eta: 0,
        goal: "AA".into(),
    };

    rec2(&report, &distance_map, closed_valves, 26, [me, elephant])
}

fn get_distance_map(report: &HashMap<Name, Valve>) -> HashMap<(Name, Name), usize> {
    let mut result = HashMap::new();
    for v1 in report.values() {
        for v2 in report.values() {
            let distance = bfs(report, v1.name.clone(), v2.name.clone()).unwrap();

            result.insert((v1.name.clone(), v2.name.clone()), distance);
            result.insert((v2.name.clone(), v1.name.clone()), distance);
        }
    }
    result
}

fn bfs(report: &HashMap<Name, Valve>, start: Name, goal: Name) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((name, distance)) = queue.pop_front() {
        if name == goal {
            return Some(distance);
        }
        visited.insert(name.clone());
        for v in report[&name].adjacent_valves.clone() {
            if !visited.contains(&v) {
                queue.push_back((v, distance + 1));
            }
        }
    }
    None
}
