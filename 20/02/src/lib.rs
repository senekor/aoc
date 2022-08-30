pub fn part1(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let (low, high) = {
            let range = tokens.next().unwrap();
            let mut iter = range.split('-');
            let low: i32 = iter.next().unwrap().parse().unwrap();
            let high = iter.next().unwrap().parse().unwrap();
            (low, high)
        };
        let req_char = tokens.next().unwrap().chars().next().unwrap();
        let password = tokens.next().unwrap();

        let mut char_count = 0;
        for c in password.chars() {
            if c == req_char {
                char_count += 1;
            }
        }
        if low <= char_count && char_count <= high {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let (low, high) = {
            let range = tokens.next().unwrap();
            let mut iter = range.split('-');
            let low: usize = iter.next().unwrap().parse().unwrap();
            let high: usize = iter.next().unwrap().parse().unwrap();
            (low, high)
        };
        let req_char = tokens.next().unwrap().chars().next().unwrap();
        let password: Vec<_> = tokens.next().unwrap().chars().collect();

        if (password[low - 1] == req_char) ^ (password[high - 1] == req_char) {
            count += 1;
        }
    }

    count
}
