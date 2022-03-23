fn main() {
    let input = include_str!("../input/input.txt");
    let (res_part1, res_part2) = day01::lib_main(input);
    println!("{}", res_part1);
    println!("{}", res_part2);
}
