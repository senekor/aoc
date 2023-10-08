struct Program(Vec<usize>);

impl std::str::FromStr for Program {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Program(s.split(',').map(|x| x.parse().unwrap()).collect()))
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0[0]).unwrap();
        for x in &self.0[1..] {
            write!(f, ",{}", x).unwrap();
        }
        Ok(())
    }
}

impl Program {
    fn execute(&mut self) {
        let mut ptr_iter = 0..;
        while let Some(ptr) = ptr_iter.next() {
            match self.0[ptr] {
                1 | 2 => {
                    let src_1 = self.0[ptr + 1];
                    let src_2 = self.0[ptr + 2];
                    let dest = self.0[ptr + 3];
                    if self.0[ptr] == 1 {
                        self.0[dest] = self.0[src_1] + self.0[src_2];
                    } else {
                        self.0[dest] = self.0[src_1] * self.0[src_2];
                    }
                    ptr_iter.nth(2);
                }
                99 => return,
                _ => panic!("unexpected opcode: {ptr}"),
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut prog: Program = input.parse().unwrap();
    prog.0[1] = 12;
    prog.0[2] = 2;
    prog.execute();
    prog.0[0]
}

pub fn part2(input: &str) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut prog: Program = input.parse().unwrap();
            prog.0[1] = noun;
            prog.0[2] = verb;
            prog.execute();
            if prog.0[0] == 19690720 {
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
        let mut prog: Program = "1,9,10,3,2,3,11,0,99,30,40,50".parse().unwrap();
        prog.execute();
        assert_eq!(prog.to_string(), "3500,9,10,70,2,3,11,0,99,30,40,50")
    }

    #[test]
    fn test_execute_2() {
        let mut prog: Program = "1,0,0,0,99".parse().unwrap();
        prog.execute();
        assert_eq!(prog.to_string(), "2,0,0,0,99")
    }

    #[test]
    fn test_execute_3() {
        let mut prog: Program = "2,3,0,3,99".parse().unwrap();
        prog.execute();
        assert_eq!(prog.to_string(), "2,3,0,6,99")
    }

    #[test]
    fn test_execute_4() {
        let mut prog: Program = "2,4,4,5,99,0".parse().unwrap();
        prog.execute();
        assert_eq!(prog.to_string(), "2,4,4,5,99,9801")
    }

    #[test]
    fn test_execute_5() {
        let mut prog: Program = "1,1,1,4,99,5,6,0,99".parse().unwrap();
        prog.execute();
        assert_eq!(prog.to_string(), "30,1,1,4,2,5,6,0,99")
    }
}
