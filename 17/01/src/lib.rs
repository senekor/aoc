pub fn part1(input: &str) -> usize {
    let len = input.len();
    input
        .chars()
        .cycle()
        .take(len + 1)
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[0] == w[1])
        .map(|w| w[0].to_string().parse::<usize>().unwrap())
        .sum()
}

pub fn part2(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<_>>();
    let half_len = chars.len() / 2;

    (0..half_len)
        .into_iter()
        .filter_map(|i| {
            if chars[i] == chars[i + half_len] {
                Some(chars[i].to_string().parse::<usize>().unwrap())
            } else {
                None
            }
        })
        .sum::<usize>()
        * 2
}
