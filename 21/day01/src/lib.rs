#![deny(missing_docs)]
//! This crate provides a solution for the advent of code puzzle: 2021, day 1.

fn add_three(a: i32, b: i32, c: i32) -> i32 {
    (a) + (b) + (c)
}

fn parse_values_to_vector(input: &str) -> Vec<i32> {
    let mut vector = Vec::new();
    for unparsed_value in input.split('\n') {
        let parsed_value = unparsed_value.parse::<i32>().unwrap();
        vector.push(parsed_value);
    }
    vector
}

fn part1(input: &str) -> i32 {
    let parsed_values = parse_values_to_vector(input);
    let mut count = 0;
    for window in parsed_values.windows(2) {
        if window[0] < window[1] {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> i32 {
    let parsed_values = parse_values_to_vector(input);
    let mut count = 0;
    for window in parsed_values.windows(4) {
        if add_three(window[0], window[1], window[2]) < add_three(window[1], window[2], window[3]) {
            count += 1;
        }
    }
    count
}

/// lib_main takes the input of the puzzle and returns the solutions for
/// both part 1 and 2 as a tuple.
pub fn lib_main(input: &str) -> (i32, i32) {
    (part1(input), part2(input))
}
