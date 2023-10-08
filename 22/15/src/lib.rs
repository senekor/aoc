#![deny(clippy::as_conversions)]

use std::{cmp::Ordering, ops::Range};

use nalgebra::Point2;
use parse::SensorReport;
use utils::Itertools;

mod parse;

type Point = Point2<i64>;

fn manhattan_distance(a: Point, b: Point) -> i64 {
    (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)).try_into().unwrap()
}

fn contains_cmp(haystack: &Range<i64>, needle: i64) -> Ordering {
    if haystack.start <= needle && needle < haystack.end {
        Ordering::Equal
    } else {
        haystack.start.cmp(&needle)
    }
}

pub fn non_beacons_at_y<const Y: i64>(input: &str) -> usize {
    let (_, reports) = parse::reports(input).unwrap();

    let mut beacons = Vec::new();
    let mut ranges = Vec::new();
    for SensorReport { sensor, beacon } in reports {
        if beacon.y == Y {
            beacons.push(beacon.x);
        }
        let dist = manhattan_distance(sensor, beacon);
        let vert_dist: i64 = sensor.y.abs_diff(Y).try_into().unwrap();
        let horiz_dist = dist - vert_dist;
        let range = sensor.x - horiz_dist..sensor.x + horiz_dist + 1;
        if range.is_empty() {
            continue;
        }
        let idx = ranges
            .binary_search_by(|a| contains_cmp(a, range.start))
            .unwrap_or_else(|i| i);
        if idx == ranges.len() {
            ranges.push(range);
            continue;
        }
        let mut merged_range = range;
        while ranges.len() != idx && merged_range.end >= ranges[idx].start {
            merged_range.start = merged_range.start.min(ranges[idx].start);
            merged_range.end = merged_range.end.max(ranges[idx].end);
            ranges.remove(idx);
        }
        ranges.insert(idx, merged_range);
    }
    beacons.sort_unstable();
    beacons.dedup();
    for beacon in beacons {
        let Ok(idx) = ranges.binary_search_by(|range| contains_cmp(range, beacon)) else {
            continue;
        };
        let range_to_split = ranges[idx].clone();
        let split_idx = idx.try_into().unwrap();
        let l = range_to_split.start..split_idx;
        let r = split_idx + 1..range_to_split.end;
        ranges[idx] = l;
        ranges.insert(idx + 1, r);
    }
    ranges
        .into_iter()
        .map(|range| usize::try_from(range.end - range.start).unwrap())
        .sum()
}

pub fn part1(input: &str) -> usize {
    non_beacons_at_y::<2_000_000>(input)
}

#[derive(Debug, Clone, Copy)]
struct Diamond {
    center: Point2<i64>,
    radius: i64,
}

impl From<SensorReport> for Diamond {
    fn from(SensorReport { sensor, beacon }: SensorReport) -> Self {
        Diamond {
            center: Point2::new(sensor.x, sensor.y),
            radius: manhattan_distance(sensor, beacon),
        }
    }
}

struct IterPerimeter {
    diamond: Diamond,
    pos: Point2<i64>, // within 0..4M
}

impl Diamond {
    fn iter_perimeter(&self) -> IterPerimeter {
        let start = Point2::new(self.center.x, self.center.y.saturating_sub(self.radius + 1));
        IterPerimeter {
            diamond: *self,
            pos: start,
        }
    }

    fn contains(&self, pos: Point2<i64>) -> bool {
        manhattan_distance(self.center, pos) <= self.radius
    }
}

#[test]
fn test_diamond_contains() {
    let d = Diamond {
        center: Point2::new(5, 5),
        radius: 3,
    };
    let within = Point2::new(6, 7);
    let outside = Point2::new(7, 7);
    assert!(d.contains(within));
    assert!(!d.contains(outside));
}

impl IterPerimeter {
    fn advance_by_with_countdown(
        &mut self,
        (x, y): (i64, i64),
        n: &mut usize,
    ) -> Option<Point2<i64>> {
        let res = self.pos;
        self.pos = Point2::new(self.pos.x + x, self.pos.y + y);
        if *n == 0 {
            return Some(res);
        }
        *n += 1;
        None
    }
}

impl Iterator for IterPerimeter {
    type Item = Point2<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        // forward to .nth(0) to avoid code duplication
        #[allow(clippy::iter_nth_zero)]
        self.nth(0)
    }

    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        if self.pos == self.diamond.center {
            return None;
        }
        if self.pos.x >= self.diamond.center.x {
            // right
            while self.pos.y < self.diamond.center.y {
                // top right
                if let Some(result) = self.advance_by_with_countdown((1, 1), &mut n) {
                    return Some(result);
                }
            }
            while self.pos.x > self.diamond.center.x {
                // bottom right
                if let Some(result) = self.advance_by_with_countdown((-1, 1), &mut n) {
                    return Some(result);
                }
            }
        }
        // left
        while self.pos.y > self.diamond.center.y {
            // bottom left
            if let Some(result) = self.advance_by_with_countdown((-1, -1), &mut n) {
                return Some(result);
            }
        }
        while self.pos.x < self.diamond.center.x {
            // top left
            if let Some(result) = self.advance_by_with_countdown((1, -1), &mut n) {
                if self.pos.x == self.diamond.center.x {
                    self.pos = self.diamond.center;
                }
                return Some(result);
            }
        }
        // end
        self.pos = self.diamond.center;
        None
    }
}

#[test]
fn test_iter_perimeter() {
    let diamond = Diamond {
        center: Point2::new(5, 5),
        radius: 2,
    };
    let expected = [
        (5, 2),
        (6, 3),
        (7, 4),
        (8, 5),
        (7, 6),
        (6, 7),
        (5, 8),
        (4, 7),
        (3, 6),
        (2, 5),
        (3, 4),
        (4, 3),
    ]
    .into_iter()
    .map(|(x, y)| Point2::new(x, y))
    .collect_vec();
    let actual = diamond.iter_perimeter().collect_vec();
    assert_eq!(actual, expected);
}

impl Diamond {
    fn find_distress_beacon_at_perimeter<const MAX_SEARCH_AREA: i64>(
        &self,
        other_diamonds: &[Diamond],
    ) -> Option<Point2<i64>> {
        let mut iter_perimeter = self.iter_perimeter().peekable();

        'perimeter_loop: while let Some(pos) = iter_perimeter.next() {
            if let Some(neg) = [pos.x, pos.y].into_iter().find(|i| i.is_negative()) {
                // eagerly skip steps that are surely in the negative
                let steps_to_skip = usize::try_from(neg.abs_diff(-1)).unwrap();
                if steps_to_skip > 0 {
                    iter_perimeter.nth(steps_to_skip - 1);
                }
                continue 'perimeter_loop;
            }
            if let Some(too_big) = [pos.x, pos.y].into_iter().find(|&i| i > MAX_SEARCH_AREA) {
                // eagerly skip steps that are surely outside the search area
                let steps_to_skip = usize::try_from(too_big.abs_diff(MAX_SEARCH_AREA + 1)).unwrap();
                if steps_to_skip > 0 {
                    iter_perimeter.nth(steps_to_skip - 1);
                }
                continue 'perimeter_loop;
            }
            for other_d in other_diamonds {
                if other_d.contains(pos) {
                    // eagerly skip steps that are in the same diamond
                    while iter_perimeter
                        .peek()
                        .map_or(false, |&p| other_d.contains(p))
                    {
                        iter_perimeter.next();
                    }
                    continue 'perimeter_loop;
                }
            }
            // no collision with other diamonds!
            return Some(pos);
        }
        None
    }
}

pub fn tuning_freq_in<const MAX_SEARCH_AREA: i64>(input: &str) -> usize {
    let (_, reports) = parse::reports(input).unwrap();
    let diamonds = reports.into_iter().map(Diamond::from).collect_vec();
    for diamond in diamonds.iter() {
        if let Some(distress_beacon) =
            diamond.find_distress_beacon_at_perimeter::<MAX_SEARCH_AREA>(&diamonds)
        {
            return usize::try_from(distress_beacon.x * 4_000_000).unwrap()
                + usize::try_from(distress_beacon.y).unwrap();
        }
    }
    unreachable!("didn't find distress signal")
}

pub fn part2(input: &str) -> usize {
    tuning_freq_in::<4_000_000>(input)
}
