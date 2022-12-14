fn is_nice_string(line: &str) -> bool {
    // rule 1: at least three vowels
    let num_values = line.matches('a').count()
        + line.matches('e').count()
        + line.matches('i').count()
        + line.matches('o').count()
        + line.matches('u').count();
    if num_values < 3 {
        return false;
    };

    // rule 2: at least one double character
    let mut has_double = false;
    let bytes = line.as_bytes();
    for idx in 1..bytes.len() {
        if bytes[idx - 1] == bytes[idx] {
            has_double = true;
            break;
        }
    }
    if !has_double {
        return false;
    };

    // rule 3: some exceptions
    if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy") {
        return false;
    };

    true
}

pub fn part1(input: &str) -> i32 {
    let mut num_nice_lines = 0;
    for line in input.split('\n') {
        if is_nice_string(line) {
            num_nice_lines += 1;
        }
    }
    num_nice_lines
}

fn is_nice_string_2(line: &str) -> bool {
    // rule 1: non-overlapping pair of double characters
    let mut has_double_pair = false;
    let bytes = line.as_bytes();
    for idx in 1..bytes.len() {
        let pair = line.get((idx - 1)..(idx + 1)).unwrap();
        if line.matches(pair).count() >= 2 {
            has_double_pair = true;
            break;
        }
    }
    if !has_double_pair {
        return false;
    }

    // rule 2: at least one double character (with another between)
    let mut has_double = false;
    for idx in 2..bytes.len() {
        if bytes[idx - 2] == bytes[idx] {
            has_double = true;
            break;
        }
    }
    if !has_double {
        return false;
    };

    true
}

pub fn part2(input: &str) -> i32 {
    let mut num_nice_lines = 0;
    for line in input.split('\n') {
        if is_nice_string_2(line) {
            num_nice_lines += 1;
        }
    }
    num_nice_lines
}
