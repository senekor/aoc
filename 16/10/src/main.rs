fn main() {
    use utils::output::Output;

    let input_dir = format!("{}/input", std::env!("CARGO_MANIFEST_DIR"));

    let input_file_name = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => String::from("input"),
    };

    let input_file_path = format!("{input_dir}/{input_file_name}.txt");
    let input = std::fs::read_to_string(input_file_path).expect("failed to read input file");

    if input_file_name == "input" {
        println!("part1: {}", aoc_16_10::part1(&input, 61, 17).output());
        println!("part2: {}", aoc_16_10::part2(&input).output());
    } else {
        println!("part1: {}", aoc_16_10::part1(&input, 5, 2).output());
        println!("part2: {}", aoc_16_10::part2(&input).output());
    }
}
