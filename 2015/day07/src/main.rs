use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Operand<'a> {
    Sig(u16),
    Wire(&'a str),
}

use Operand::*;

#[derive(Copy, Clone)]
enum Operation<'a> {
    Cons(Operand<'a>),
    And(Operand<'a>, Operand<'a>),
    Or(Operand<'a>, Operand<'a>),
    Lshf(Operand<'a>, Operand<'a>),
    Rshf(Operand<'a>, Operand<'a>),
    Not(Operand<'a>),
}

use Operation::*;

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

type Guide<'a> = HashMap<&'a str, Operation<'a>>;

fn eval_oprns<'a>(
    guide: &Guide<'a>,
    wires: &mut HashMap<&'a str, u16>,
    operand: &Operand<'a>,
) -> u16 {
    match operand {
        Sig(s) => *s,
        Wire(w) => eval_wire(guide, wires, w),
    }
}

fn eval_wire<'a>(guide: &Guide<'a>, wires: &mut HashMap<&'a str, u16>, wire: &'a str) -> u16 {
    if let Some(sig) = wires.get(wire) {
        return *sig;
    };
    let new_sig = match guide.get(wire).unwrap() {
        Cons(a) => eval_oprns(guide, wires, a),
        And(a, b) => eval_oprns(guide, wires, a) & eval_oprns(guide, wires, b),
        Or(a, b) => eval_oprns(guide, wires, a) | eval_oprns(guide, wires, b),
        Lshf(a, b) => eval_oprns(guide, wires, a) << eval_oprns(guide, wires, b),
        Rshf(a, b) => eval_oprns(guide, wires, a) >> eval_oprns(guide, wires, b),
        Not(a) => !eval_oprns(guide, wires, a),
    };
    wires.insert(wire, new_sig);
    new_sig
}

fn part1(input: &str) {
    let num_instructions = input.split('\n').count();
    let mut guide = HashMap::with_capacity(num_instructions);
    for line in input.split('\n') {
        let instr = parse_instr(line);
        guide.insert(instr.dest, instr.op);
    }

    let mut wires = HashMap::with_capacity(num_instructions);
    let a_val = eval_wire(&guide, &mut wires, "a");
    println!("signal to wire 'a': {}", a_val);
}

fn part2(input: &str) {
    let num_instructions = input.split('\n').count();
    let mut guide = HashMap::with_capacity(num_instructions);
    for line in input.split('\n') {
        let instr = parse_instr(line);
        guide.insert(instr.dest, instr.op);
    }

    let mut wires = HashMap::with_capacity(num_instructions);
    let a_val = eval_wire(&guide, &mut wires, "a");
    wires = HashMap::with_capacity(num_instructions);
    wires.insert("b", a_val);
    let a = eval_wire(&guide, &mut wires, "a");
    println!("signal to wire 'a': {}", a);
}

fn main() {
    let input = include_str!("../input/input.txt");

    part1(input);
    part2(input);
}
