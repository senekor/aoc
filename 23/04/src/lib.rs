pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|card| {
            let mut iter = card.split('|').map(|part| {
                part.split_ascii_whitespace()
                    .map(|n| n.parse::<u32>())
                    .filter_map(Result::ok)
            });
            let winning = iter.next().unwrap().collect::<Vec<_>>();
            (1 << iter.next().unwrap().filter(|n| winning.contains(n)).count()) / 2
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut cards = input
        .lines()
        .map(|card| {
            let mut iter = card.split('|').map(|part| {
                part.split_ascii_whitespace()
                    .map(|n| n.parse::<u32>())
                    .filter_map(Result::ok)
            });
            let winning = iter.next().unwrap().collect::<Vec<_>>();
            let hits = iter.next().unwrap().filter(|n| winning.contains(n)).count();
            (1, hits)
        })
        .collect::<Vec<_>>();
    for i in 0..cards.len() {
        let (copies, hits) = cards[i];
        for won_copy in &mut cards[i + 1..i + 1 + hits] {
            won_copy.0 += copies;
        }
    }
    cards.into_iter().map(|t| t.0).sum()
}
