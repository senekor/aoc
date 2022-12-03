fn get_score(game: &str) -> u32 {
    let (other, mine) = game.split_once(' ').unwrap();
    let shape_score = match mine {
        "X" => 1,
        "Y" => 2,
        _ => 3,
    };
    let outcome_score = match (other, mine) {
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        _ => 0,
    };
    shape_score + outcome_score
}

pub fn part1(input: &str) -> u32 {
    input.lines().map(get_score).sum()
}

fn get_score_2(game: &str) -> u32 {
    let (other, mine) = game.split_once(' ').unwrap();
    let outcome_score = match mine {
        "X" => 0,
        "Y" => 3,
        _ => 6,
    };
    let shape_score = match (other, mine) {
        ("A", "Y") | ("B", "X") | ("C", "Z") => 1,
        ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
        _ => 3,
    };
    shape_score + outcome_score
}

pub fn part2(input: &str) -> u32 {
    input.lines().map(get_score_2).sum()
}
