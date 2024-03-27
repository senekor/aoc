use winnow::{
    ascii::{alpha1, dec_uint},
    combinator::{separated, separated_pair},
    PResult, Parser,
};

use crate::{Game, Revelation};

pub fn game(mut input: &str) -> PResult<Game> {
    let input = &mut input;
    "Game ".parse_next(input)?;
    let id = dec_uint::<_, u64, _>(input)? as usize;
    ": ".parse_next(input)?;
    let revelations = separated(1.., revelation, "; ").parse_next(input)?;
    Ok(Game { id, revelations })
}

fn revelation(input: &mut &str) -> PResult<Revelation> {
    let num_color = separated_pair(dec_uint::<_, u64, _>.map(|n| n as usize), " ", alpha1);
    let pairs: Vec<_> = separated(1.., num_color, ", ").parse_next(input)?;

    let mut res = Revelation::default();
    for (num, color) in pairs {
        match color {
            "red" => res.red = num,
            "green" => res.green = num,
            "blue" => res.blue = num,
            _ => panic!("unexpected color '{color}' while parsing revelation"),
        }
    }
    Ok(res)
}
