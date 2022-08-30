fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", aoc_16_02::part1(input));
    println!("{:#x}", aoc_16_02::part2(input));
}
