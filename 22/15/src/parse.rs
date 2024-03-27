use winnow::{
    ascii::{digit1, newline},
    combinator::{opt, preceded, separated, separated_pair},
    PResult, Parser,
};

use crate::Point;

fn i64(input: &mut &str) -> PResult<i64> {
    (opt('-'), digit1)
        .recognize()
        .map(|digs: &str| digs.parse().unwrap())
        .parse_next(input)
}

fn coordinate(c: char) -> impl Fn(&mut &str) -> PResult<i64> {
    move |input: &mut &str| preceded((c, '='), i64).parse_next(input)
}

fn point(input: &mut &str) -> PResult<Point> {
    separated_pair(coordinate('x'), ", ", coordinate('y'))
        .map(|(x, y)| Point::new(x, y))
        .parse_next(input)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SensorReport {
    pub sensor: Point,
    pub beacon: Point,
}

fn sensor_report(input: &mut &str) -> PResult<SensorReport> {
    (
        preceded("Sensor at ", point),
        preceded(": closest beacon is at ", point),
    )
        .map(|(sensor, beacon)| SensorReport { sensor, beacon })
        .parse_next(input)
}

pub(crate) fn reports(input: &mut &str) -> PResult<Vec<SensorReport>> {
    separated(.., sensor_report, newline).parse_next(input)
}

#[test]
fn test_reports() {
    let mut input = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
";
    let reports = reports(&mut input).unwrap();
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
