fn line_to_calibration_value(line: &str) -> u32 {
    let mut digits = line
        .chars()
        .filter(char::is_ascii_digit)
        .map(|c| (c as u8) - b'0');
    let first = digits.next().unwrap();
    let last = digits.last().unwrap_or(first);
    (first * 10 + last) as u32
}

pub fn part1(input: &str) -> u32 {
    input.lines().map(line_to_calibration_value).sum()
}

struct Suffixes<'a>(&'a str);

impl<'a> Iterator for Suffixes<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        let res = self.0;
        // cut off first ascii char for next iteration.
        // would panic at invalid utf-8 byte offset if input was not ascii.
        self.0 = self.0.split_at(1).1;
        Some(res)
    }
}

static STR_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_prefix_as_digit(s: &str) -> Option<u8> {
    if let Ok(digit) = s[0..1].parse() {
        // first character is a regular valid digit
        return Some(digit);
    }
    STR_DIGITS
        .iter()
        .enumerate()
        .find(|(_, digit)| s.starts_with(*digit))
        .map(|(idx, _)| idx as u8 + 1)
}

fn line_to_calibration_value_2(line: &str) -> u32 {
    let mut digits = Suffixes(line).flat_map(parse_prefix_as_digit);
    let first = digits.next().unwrap();
    let last = digits.last().unwrap_or(first);
    (first * 10 + last) as u32
}

pub fn part2(input: &str) -> u32 {
    input.lines().map(line_to_calibration_value_2).sum()
}
