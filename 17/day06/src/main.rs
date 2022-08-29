use day06::{part1, part2};

fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", part1::<16>(input));
    println!("{}", part2::<16>(input));
}
