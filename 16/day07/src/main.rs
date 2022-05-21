use day07::{part1, part2};

fn main() {
    println!("silvia:");
    let input = include_str!("../input/input_silvia.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
    println!("remo:");
    let input_remo = include_str!("../input/input_remo.txt");
    println!("{}", part1(input_remo));
    println!("{}", part2(input_remo));
}
