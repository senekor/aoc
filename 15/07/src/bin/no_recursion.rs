use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone)]
enum Operand<'a> {
    Sig(u16),
    Wire(&'a str),
}

#[derive(Copy, Clone)]
enum Operation<'a> {
    Cons(Operand<'a>),
    And(Operand<'a>, Operand<'a>),
    Or(Operand<'a>, Operand<'a>),
    Lshf(Operand<'a>, Operand<'a>),
    Rshf(Operand<'a>, Operand<'a>),
    Not(Operand<'a>),
}

#[derive(Copy, Clone)]
struct Instr<'a> {
    dest: &'a str,
    op: Operation<'a>,
}

fn to_operand(token: &str) -> Operand {
    match token.parse::<u16>() {
        Ok(signal) => Operand::Sig(signal),
        Err(_) => Operand::Wire(token),
    }
}

fn parse_instr(line: &str) -> Instr {
    let mut tokens = line.split(' ');

    let first = tokens.next().unwrap();
    let second = tokens.next().unwrap();

    if second == "->" {
        let cons = to_operand(first);
        let dest = tokens.next().unwrap();
        assert_eq!(None, tokens.next());
        return Instr {
            dest,
            op: Operation::Cons(cons),
        };
    }

    if first == "NOT" {
        let operand = to_operand(second);
        assert_eq!("->", tokens.next().unwrap());
        let dest = tokens.next().unwrap();
        assert_eq!(None, tokens.next());
        return Instr {
            dest,
            op: Operation::Not(operand),
        };
    }

    let left_operand = to_operand(first);
    let operator = second;
    let right_operand = to_operand(tokens.next().unwrap());
    assert_eq!("->", tokens.next().unwrap());
    let dest = tokens.next().unwrap();
    assert_eq!(None, tokens.next());

    match operator {
        "AND" => Instr {
            dest,
            op: Operation::And(left_operand, right_operand),
        },
        "OR" => Instr {
            dest,
            op: Operation::Or(left_operand, right_operand),
        },
        "LSHIFT" => Instr {
            dest,
            op: Operation::Lshf(left_operand, right_operand),
        },
        "RSHIFT" => Instr {
            dest,
            op: Operation::Rshf(left_operand, right_operand),
        },
        i => panic!("unrecognized instruction: {}", i),
    }
}

fn to_dep<'a>(operand: &'a Operand) -> Option<&'a str> {
    match operand {
        Operand::Sig(_) => None,
        Operand::Wire(a) => Some(a),
    }
}

fn get_deps<'a>(instr: &'a Instr<'a>) -> Vec<&'a str> {
    let (a, b) = match &instr.op {
        Operation::Cons(a) => (to_dep(a), None),
        Operation::Not(a) => (to_dep(a), None),
        Operation::And(a, b) => (to_dep(a), to_dep(b)),
        Operation::Or(a, b) => (to_dep(a), to_dep(b)),
        Operation::Lshf(a, b) => (to_dep(a), to_dep(b)),
        Operation::Rshf(a, b) => (to_dep(a), to_dep(b)),
    };
    let mut v = Vec::new();
    if let Some(some_a) = a {
        v.push(some_a)
    };
    if let Some(some_b) = b {
        v.push(some_b)
    };
    v
}

fn sort_guide(guide: &mut [Instr]) {
    let mut known_deps: HashSet<&str> = HashSet::new();
    for i in 0..guide.len() {
        for k in i..guide.len() {
            let instr = guide[k];
            let mut all_deps_present = true;
            for dep in get_deps(&instr) {
                if known_deps.get(dep).is_none() {
                    all_deps_present = false
                }
            }
            if all_deps_present {
                known_deps.insert(instr.dest);
                guide.swap(i, k);
                break;
            }
        }
    }
}

fn to_sig<'a>(circ: &HashMap<&'a str, u16>, operand: &'a Operand) -> u16 {
    match operand {
        Operand::Sig(signal) => *signal,
        Operand::Wire(wire) => *circ.get(wire).unwrap(),
    }
}

fn eval_op<'a>(circ: &HashMap<&'a str, u16>, op: &Operation<'a>) -> u16 {
    match op {
        Operation::Cons(a) => to_sig(circ, a),
        Operation::Not(a) => !to_sig(circ, a),
        Operation::And(a, b) => to_sig(circ, a) & to_sig(circ, b),
        Operation::Or(a, b) => to_sig(circ, a) | to_sig(circ, b),
        Operation::Lshf(a, b) => to_sig(circ, a) << to_sig(circ, b),
        Operation::Rshf(a, b) => to_sig(circ, a) >> to_sig(circ, b),
    }
}

fn build_circuit<'a>(guide: &[Instr<'a>]) -> HashMap<&'a str, u16> {
    let mut circuit = HashMap::with_capacity(guide.len());
    for instr in guide {
        let signal = eval_op(&circuit, &instr.op);
        circuit.insert(instr.dest, signal);
    }
    circuit
}

fn part1(input: &str) {
    let num_instructions = input.split('\n').count();
    let mut guide: Vec<Instr> = Vec::with_capacity(num_instructions);
    for line in input.split('\n') {
        let instr = parse_instr(line);
        guide.push(instr);
    }

    sort_guide(&mut guide);

    let wires = build_circuit(&guide);
    println!("signal to wire 'a': {}", wires.get("a").unwrap());
}

fn build_circuit_2<'a>(guide: &[Instr<'a>]) -> HashMap<&'a str, u16> {
    let mut circuit = HashMap::with_capacity(guide.len());
    for instr in guide {
        let signal = eval_op(&circuit, &instr.op);
        circuit.insert(instr.dest, signal);
    }
    let sig_a = *circuit.get("a").unwrap();
    circuit = HashMap::with_capacity(guide.len());
    let mut new_guide = guide.to_owned();
    for item in &mut new_guide {
        if item.dest == "b" {
            item.op = Operation::Cons(Operand::Sig(sig_a));
        }
    }
    for instr in new_guide {
        let signal = eval_op(&circuit, &instr.op);
        circuit.insert(instr.dest, signal);
    }
    circuit
}

fn part2(input: &str) {
    let num_instructions = input.split('\n').count();
    let mut guide: Vec<Instr> = Vec::with_capacity(num_instructions);
    for line in input.split('\n') {
        let instr = parse_instr(line);
        guide.push(instr);
    }

    sort_guide(&mut guide);

    let wires = build_circuit_2(&guide);
    println!("signal to wire 'a': {}", wires.get("a").unwrap());
}

/// This version sorts the instructions based on dependencies,
/// such that the instructions can be executed sequentially without
/// running into undefined dependencies. This was done because
/// I was unsuccessfully fighting the borrow checker while trying
/// to mutate a hash map recursively, which is necessary to efficiently
/// execute the instructions unsorted, as they are provided.
///
/// The newer version uses recursion successfully, making the whole
/// program much more readable and straight-forward.
fn main() {
    let input = include_str!("../../input/input.txt");

    part1(input);
    part2(input);
}
