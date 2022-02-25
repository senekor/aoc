fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", day02::part1(input));
    println!("{:#x}", day02::part2(input));
}
