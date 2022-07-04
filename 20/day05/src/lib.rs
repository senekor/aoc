fn binary_space_partition(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold((0, usize::pow(2, s.len() as u32)), |(l, r), b| match b {
            b'F' | b'L' => (l, r - ((r - l) / 2)),
            _ => (l + (r - l) / 2, r),
        })
        .0
}

fn parse_seat_id(s: &str) -> usize {
    let row = binary_space_partition(&s[..7]);
    let col = binary_space_partition(&s[7..]);
    row * 8 + col
}

pub fn part1(input: &str) -> usize {
    input.lines().map(parse_seat_id).max().unwrap()
}

pub fn part2(input: &str) -> usize {
    let mut ids: Vec<_> = input.lines().map(parse_seat_id).collect();
    ids.sort_unstable();
    ids.windows(2).find(|w| w[1] - w[0] == 2).unwrap()[0] + 1
}
