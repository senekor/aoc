mod parse;

#[derive(Debug, Default)]
struct Revelation {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    id: usize,
    revelations: Vec<Revelation>,
}

pub fn f<const RED: usize, const GREEN: usize, const BLUE: usize>(input: &str) -> usize {
    input
        .lines()
        .map(parse::game)
        .map(Result::unwrap)
        .filter(|game| {
            game.revelations
                .iter()
                .all(|revel| revel.red <= RED && revel.green <= GREEN && revel.blue <= BLUE)
        })
        .map(|game| game.id)
        .sum()
}

pub fn part1(input: &str) -> usize {
    f::<12, 13, 14>(input)
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse::game)
        .map(Result::unwrap)
        .map(|game| {
            let min_game = game
                .revelations
                .into_iter()
                .fold(Revelation::default(), |acc, next| Revelation {
                    red: acc.red.max(next.red),
                    green: acc.green.max(next.green),
                    blue: acc.blue.max(next.blue),
                });
            min_game.red * min_game.green * min_game.blue
        })
        .sum()
}
