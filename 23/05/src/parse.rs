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

pub fn seed_numbers(input: &mut &str) -> PResult<Vec<Range<Seed>>> {
    "seeds: ".parse_next(input)?;
    separated(1.., number.map(Number::to_single_range), ' ').parse_next(input)
}

#[test]
fn can_parse_seed_numbers() {
    let nums = seed_numbers(&mut "seeds: 79 14 55 13").unwrap();
    let expected: Vec<Range<Seed>> = [79, 14, 55, 13]
        .into_iter()
        .map(Number::from)
        .map(Number::to_single_range)
        .collect();
    assert_eq!(nums, expected);
}

pub fn seed_ranges(input: &mut &str) -> PResult<Vec<Range<Seed>>> {
    "seeds: ".parse_next(input)?;
    separated(
        1..,
        (dec_uint::<_, Int, _>.map(Number::from), ' ', dec_uint)
            .map(|(start, _, length)| Range { start, length }),
        ' ',
    )
    .parse_next(input)
}

#[test]
fn can_parse_seed_ranges() {
    let nums = seed_ranges(&mut "seeds: 79 14 55 13").unwrap();
    let expected: Vec<Range<Seed>> = [(79, 14), (55, 13)]
        .into_iter()
        .map(|(start, length)| Range {
            start: Number::from(start),
            length,
        })
        .collect();
    assert_eq!(nums, expected);
}

fn map_range<Src: Category, Dest: Category>(input: &mut &str) -> PResult<MapRange<Src, Dest>> {
    let destination_range_start = number(input)?;
    ' '.parse_next(input)?;
    let source_range_start = number(input)?;
    ' '.parse_next(input)?;
    let length = dec_uint(input)?;

    let dest = Range {
        start: destination_range_start,
        length,
    };
    let src = Range {
        start: source_range_start,
        length,
    };
    Ok(MapRange { src, dest })
}

#[test]
fn can_parse_map_range() {
    let range = map_range(&mut "50 98 2").unwrap();
    let expected = MapRange::<Seed, Soil> {
        src: Range {
            start: 98.into(),
            length: 2,
        },
        dest: Range {
            start: 50.into(),
            length: 2,
        },
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
        separated(1.., map_range, newline),
    )
        .parse_next(input)
        .map(|(_, mut ranges): (_, Vec<_>)| {
            ranges.sort();
            Map { ranges }
        })
}

#[test]
fn can_parse_map() {
    let map = map(&mut "seed-to-soil map:\n50 98 2\n52 50 48").unwrap();
    let expected = Map::<Seed, Soil> {
        ranges: vec![
            MapRange {
                src: Range {
                    start: 50.into(),
                    length: 48,
                },
                dest: Range {
                    start: 52.into(),
                    length: 48,
                },
            },
            MapRange {
                src: Range {
                    start: 98.into(),
                    length: 2,
                },
                dest: Range {
                    start: 50.into(),
                    length: 2,
                },
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
        seed_to_soil: map(input)?,
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
    let sample = include_str!("../input/sample.txt");
    let mut input = sample.split_once("\n\n").unwrap().1;
    almanac(&mut input).unwrap();
}
