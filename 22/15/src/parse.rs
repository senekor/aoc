use utils::{
    nom::{
        bytes::complete::tag,
        character::complete::{char, newline},
        combinator::map,
        multi::separated_list0,
        sequence::{pair, preceded, separated_pair},
        IResult,
    },
    parse::integer::i64,
};

use crate::Point;

fn coordinate(c: char) -> impl Fn(&str) -> IResult<&str, i64> {
    move |input: &str| preceded(pair(char(c), char('=')), i64)(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(coordinate('x'), tag(", "), coordinate('y')),
        |(x, y)| Point::new(x, y),
    )(input)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SensorReport {
    pub sensor: Point,
    pub beacon: Point,
}

fn sensor_report(input: &str) -> IResult<&str, SensorReport> {
    map(
        pair(
            preceded(tag("Sensor at "), point),
            preceded(tag(": closest beacon is at "), point),
        ),
        |(sensor, beacon)| SensorReport { sensor, beacon },
    )(input)
}

pub(crate) fn reports(input: &str) -> IResult<&str, Vec<SensorReport>> {
    separated_list0(newline, sensor_report)(input)
}

#[test]
fn test_reports() {
    let input = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
";
    let (_, reports) = reports(input).unwrap();
    let expected = vec![
        SensorReport {
            sensor: Point::new(2, 18),
            beacon: Point::new(-2, 15),
        },
        SensorReport {
            sensor: Point::new(9, 16),
            beacon: Point::new(10, 16),
        },
    ];
    assert_eq!(reports, expected)
}
