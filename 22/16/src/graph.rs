use std::io::Write;

use crate::parse;

pub fn from(mut input: &str) -> String {
    let mut buffer = Vec::new();
    writeln!(buffer, "graph volcano {{").unwrap();
    // writeln!(buffer, "layout=neato").unwrap();
    let report = parse::report(&mut input).unwrap();
    for (name, valve) in report {
        for neighbor in valve.adjacent_valves {
            if name < neighbor {
                writeln!(buffer, "{name} -- {neighbor};").unwrap();
            }
        }
    }
    writeln!(buffer, "}}").unwrap();
    String::from_utf8(buffer).unwrap()
}
