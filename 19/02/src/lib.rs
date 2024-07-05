use intcode_computer::IntcodeComputer;

pub fn part1(input: &str) -> i64 {
    let mut prog: IntcodeComputer = input.parse().unwrap();
    unsafe { prog.get_raw_memory()[1] = 12 };
    unsafe { prog.get_raw_memory()[2] = 2 };
    prog.execute();
    unsafe { prog.get_raw_memory()[0] }
}

pub fn part2(input: &str) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut prog: IntcodeComputer = input.parse().unwrap();
            unsafe { prog.get_raw_memory()[1] = noun };
            unsafe { prog.get_raw_memory()[2] = verb };
            prog.execute();
            if unsafe { prog.get_raw_memory()[0] } == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("no noun verb combination found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_1() {
        let mut prog: IntcodeComputer = "1,9,10,3,2,3,11,0,99,30,40,50".parse().unwrap();
        prog.execute();
        assert_eq!(prog.to_string(), "3500,9,10,70,2,3,11,0,99,30,40,50")
    }

    #[test]
    fn test_execute_2() {
        let mut prog: IntcodeComputer = "1,0,0,0,99".parse().unwrap();
        prog.execute();
        assert_eq!(prog.to_string(), "2,0,0,0,99")
    }

    #[test]
    fn test_execute_3() {
        let mut prog: IntcodeComputer = "2,3,0,3,99".parse().unwrap();
        prog.execute();
        assert_eq!(prog.to_string(), "2,3,0,6,99")
    }

    #[test]
    fn test_execute_4() {
        let mut prog: IntcodeComputer = "2,4,4,5,99,0".parse().unwrap();
        prog.execute();
        assert_eq!(prog.to_string(), "2,4,4,5,99,9801")
    }

    #[test]
    fn test_execute_5() {
        let mut prog: IntcodeComputer = "1,1,1,4,99,5,6,0,99".parse().unwrap();
        prog.execute();
        assert_eq!(prog.to_string(), "30,1,1,4,2,5,6,0,99")
    }
}
