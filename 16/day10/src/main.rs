use day10::{part1, part2};

fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", part1(input, 61, 17));
    println!("{}", part2(input));
}
