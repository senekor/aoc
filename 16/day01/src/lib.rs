mod santa;
use santa::Santa;

mod instruction;

pub fn part1(input: &str) -> i32 {
    let mut santa = Santa::default();
    let instructions = input
        .split(", ")
        .map(|instr| instr.parse().unwrap())
        .collect::<Vec<_>>();

    for instruction in instructions {
        santa.execute(instruction);
    }

    santa.get_distance()
}

pub fn part2(input: &str) -> i32 {
    let mut santa = Santa::default();
    let instructions = input
        .split(", ")
        .map(|instr| instr.parse().unwrap())
        .collect::<Vec<_>>();

    for instruction in instructions {
        santa.execute(instruction);
    }

    santa.get_first_visited_twice_distance()
}
