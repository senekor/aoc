use std::collections::HashMap;

use winnow::{
    ascii::{alpha1, digit1, newline},
    combinator::{alt, preceded, separated},
    PResult, Parser,
};

use crate::{Name, Valve};

fn valve_name(input: &mut &str) -> PResult<Name> {
    alpha1
        .verify(|n: &str| n.len() == 2)
        .map(String::from)
        .parse_next(input)
}

fn flow_rate(input: &mut &str) -> PResult<usize> {
    preceded("flow rate=", digit1.map(|d: &str| d.parse().unwrap())).parse_next(input)
}

fn adjacent_valves(input: &mut &str) -> PResult<Vec<Name>> {
    separated(1.., valve_name, ", ").parse_next(input)
}

fn valve(input: &mut &str) -> PResult<Valve> {
    let _ = "Valve ".parse_next(input)?;
    let name = valve_name(input)?;
    let _ = " has ".parse_next(input)?;
    let flow_rate = flow_rate(input)?;
    let _ = alt(("; tunnels lead to valves ", "; tunnel leads to valve ")).parse_next(input)?;
    let adjacent_valves = adjacent_valves(input)?;
    Ok(Valve {
        name,
        flow_rate,
        adjacent_valves,
    })
}

pub(crate) fn report(input: &mut &str) -> PResult<HashMap<Name, Valve>> {
    separated(.., valve.map(|v| (v.name.clone(), v)), newline).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valve() {
        let mut input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let valve = valve.parse_next(&mut input).unwrap();
        assert_eq!(
            valve,
            Valve {
                name: "AA".into(),
                flow_rate: 0,
                adjacent_valves: vec!["DD".into(), "II".into(), "BB".into()],
            },
        )
    }
}
