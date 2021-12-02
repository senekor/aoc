use itertools::*;
use std::panic;

#[derive(Clone, Debug)]
enum MyEnum {
    One,
    Two,
    Three,
}
use MyEnum::*;

fn str_to_enum(s: &str) -> MyEnum {
    match s {
        "one" => One,
        "two" => Two,
        "three" => Three,
        x => panic!("unknown direction {}", x),
    }
}

#[derive(Clone, Debug)]
struct ParsedLine {
    enum_val: MyEnum,
    str: String,
    num: i32,
}
// or, if the line is just one primitive value:
// type ParsedLine = i32;

fn parse_line(line: &str) -> ParsedLine {
    let mut iter = line.split(" ");
    ParsedLine {
        enum_val: str_to_enum(iter.next().unwrap()),
        str: iter.next().unwrap().to_string(),
        num: iter.next().unwrap().parse().unwrap(),
    }
}

type ParsedInput = Vec<ParsedLine>;
// ----- UNCOMMENT IF YOU NEED A HASHMAP -----
// type Key = String;
// type Val = ParsedLine;
// type ParsedInput = HashMap<Key, Val>;

// fn input_to_map(input: &str) -> ParsedInput {
//     let mut hm = HashMap::new();
//     input.lines().map(|line| {
//         let parsed_line = parse_line(line);
//         hm.insert(todo_, parsed_line)
//     });
//     return hm;
// }

fn part1(input: &ParsedInput) {
    println!("{:?}", input)
}

fn part2(input: &ParsedInput) {
    todo!();
    println!("{:?}", input)
}

fn main() {
    let input = include_str!("../input/input.txt");

    let parsed_input: ParsedInput = input.lines().map(parse_line).collect_vec();
    // let parsed_input: ParsedInput = input_to_map(input);

    part1(&parsed_input);

    part2(&parsed_input);
}
