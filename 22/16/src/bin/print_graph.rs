fn main() {
    let input = include_str!("../../input/sample.txt");
    let graph = aoc_22_16::graph::from(input);
    print!("{graph}");
}
