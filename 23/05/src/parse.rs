use winnow::{
    ascii::{dec_uint, newline, Caseless},
    combinator::{preceded, separated},
    PResult, Parser,
};

use super::*;

fn number<T: Category>(input: &mut &str) -> PResult<Number<T>> {
    let number: Int = dec_uint(input)?;
    Ok(Number::from(number))
}

fn seed_numbers(input: &mut &str) -> PResult<Vec<Number<Seed>>> {
    "seeds: ".parse_next(input)?;
    separated(1.., number, ' ').parse_next(input)
}

#[test]
fn can_parse_seed_numbers() {
    let nums = seed_numbers(&mut "seeds: 79 14 55 13").unwrap();
    let expected: Vec<Number<Seed>> = [79, 14, 55, 13].into_iter().map(Number::from).collect();
    assert_eq!(nums, expected);
}

fn range<Src: Category, Dest: Category>(input: &mut &str) -> PResult<Range<Src, Dest>> {
    let destination_range_start = number(input)?;
    ' '.parse_next(input)?;
    let source_range_start = number(input)?;
    ' '.parse_next(input)?;
    let range_length = dec_uint(input)?;

    Ok(Range {
        destination_range_start,
        source_range_start,
        range_length,
    })
}

#[test]
fn can_parse_range() {
    let range = range(&mut "50 98 2").unwrap();
    let expected = Range::<Seed, Soil> {
        destination_range_start: 50.into(),
        source_range_start: 98.into(),
        range_length: 2,
    };
    assert_eq!(range, expected);
}

fn recognize_category<'a, T: Category>(input: &mut &'a str) -> PResult<&'a str> {
    (Caseless(T::NAME)).parse_next(input)
}

#[test]
fn can_recognize_category() {
    recognize_category::<Seed>(&mut "seed").unwrap();
}

fn recognize_map_header<'a, Src: Category, Dest: Category>(
    input: &mut &'a str,
) -> PResult<&'a str> {
    (
        recognize_category::<Src>,
        "-to-",
        recognize_category::<Dest>,
        " map:\n",
    )
        .recognize()
        .parse_next(input)
}

#[test]
fn can_recognize_map_header() {
    recognize_map_header::<Seed, Soil>(&mut "seed-to-soil map:\n").unwrap();
}

fn map<Src: Category, Dest: Category>(input: &mut &str) -> PResult<Map<Src, Dest>> {
    (
        recognize_map_header::<Src, Dest>,
        separated(1.., range, newline),
    )
        .parse_next(input)
        .map(|(_, ranges)| Map { ranges })
}

#[test]
fn can_parse_map() {
    let map = map(&mut "seed-to-soil map:\n50 98 2\n52 50 48").unwrap();
    let expected = Map::<Seed, Soil> {
        ranges: vec![
            Range {
                destination_range_start: 50.into(),
                source_range_start: 98.into(),
                range_length: 2,
            },
            Range {
                destination_range_start: 52.into(),
                source_range_start: 50.into(),
                range_length: 48,
            },
        ],
    };
    assert_eq!(map, expected);
}

pub fn almanac(input: &mut &str) -> PResult<Almanac> {
    fn newline_prefixed_map<Src: Category, Dest: Category>(
        input: &mut &str,
    ) -> PResult<Map<Src, Dest>> {
        preceded("\n\n", map).parse_next(input)
    }
    Ok(Almanac {
        seeds: seed_numbers(input)?,
        seed_to_soil: newline_prefixed_map(input)?,
        soil_to_fertilizer: newline_prefixed_map(input)?,
        fertilizer_to_water: newline_prefixed_map(input)?,
        water_to_light: newline_prefixed_map(input)?,
        light_to_temperature: newline_prefixed_map(input)?,
        temperature_to_humidity: newline_prefixed_map(input)?,
        humidity_to_location: newline_prefixed_map(input)?,
    })
}

#[test]
fn can_recognize_almanac() {
    let mut sample = include_str!("../input/sample.txt");
    almanac(&mut sample).unwrap();
}
