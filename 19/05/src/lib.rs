use intcode_computer::IntcodeComputer;

fn test(input: &str, n: i64) -> i64 {
    let mut computer: IntcodeComputer = input.parse().unwrap();
    computer.add_input(n);
    let output = computer.execute();
    assert!(
        output[..output.len() - 1].iter().all(|&x| x == 0),
        "all output except the last must be zero"
    );
    output.last().unwrap().to_owned()
}

pub fn part1(input: &str) -> i64 {
    test(input, 1)
}

pub fn part2(input: &str) -> i64 {
    test(input, 5)
}
