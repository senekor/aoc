pub fn part1(input: &str) -> usize {
    let num_lines = input.split('\n').count();
    let num_code_chars = input.len() - num_lines + 1;

    let mut num_memory_chars = 0;
    for line in input.split('\n') {
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                '"' => {}
                '\\' => {
                    if chars.next().unwrap() == 'x' {
                        //consume hex code
                        chars.next();
                        chars.next();
                    }
                    num_memory_chars += 1
                }
                _ => num_memory_chars += 1,
            }
        }
    }

    num_code_chars - num_memory_chars
}

pub fn part2(input: &str) -> usize {
    let num_lines = input.split('\n').count();
    let num_code_chars = input.len() - num_lines + 1;

    let mut num_new_chars = 0;
    for line in input.split('\n') {
        num_new_chars += 2; // enclosing ""
        for c in line.chars() {
            match c {
                '"' => num_new_chars += 2,
                '\\' => num_new_chars += 2,
                _ => num_new_chars += 1,
            }
        }
    }

    num_new_chars - num_code_chars
}
