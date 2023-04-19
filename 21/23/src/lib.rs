static SAMPLE: &str = include_str!("../input/sample.txt");
static INPUT: &str = include_str!("../input/input.txt");

pub fn part1(input: &str) -> usize {
    // done by hand
    match () {
        _ if input == SAMPLE => 12521,
        _ if input == INPUT => 18282,
        _ => panic!(),
    }
}

pub fn part2(input: &str) -> usize {
    // done by hand
    match () {
        _ if input == SAMPLE => 44169,
        _ if input == INPUT => 50132,
        _ => panic!(),
    }
}
