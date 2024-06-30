use intcode_computer::IntcodeComputer;

pub fn part1(input: &str) -> i64 {
    let mut computer: IntcodeComputer = input.parse().unwrap();
    computer.add_input(1);
    let output = computer.execute();
    assert_eq!(output.len(), 1);
    output[0]
}

pub fn part2(input: &str) -> i64 {
    let mut computer: IntcodeComputer = input.parse().unwrap();
    computer.add_input(2);
    let output = computer.execute();
    assert_eq!(output.len(), 1);
    output[0]
}

#[cfg(test)]
mod tests {
    use intcode_computer::IntcodeComputer;

    #[test]
    fn sample_19_09_1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let output = input.parse::<IntcodeComputer>().unwrap().execute();
        let expected: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(output, expected);
    }
}
