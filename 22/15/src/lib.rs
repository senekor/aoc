use std::{cmp::Ordering, ops::Range};

use nalgebra::Point2;
use parse::SensorReport;

mod parse;

type Point = Point2<i32>;

fn manhattan_distance(a: Point, b: Point) -> u32 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn contains_cmp(haystack: &Range<i32>, needle: i32) -> Ordering {
    if haystack.start <= needle && needle < haystack.end {
        Ordering::Equal
    } else {
        haystack.start.cmp(&needle)
    }
}

pub fn non_beacons_at_y<const Y: u32>(input: &str) -> usize {
    let (_, reports) = parse::reports(input).unwrap();

    let mut beacons = Vec::new();
    let mut ranges = Vec::new();
    for SensorReport { sensor, beacon } in reports {
        if beacon.y == Y as i32 {
            beacons.push(beacon.x);
        }
        let dist = manhattan_distance(sensor, beacon);
        let vert_dist = sensor.y.abs_diff(Y as i32) as i32;
        let horiz_dist = dist as i32 - vert_dist;
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
        let Ok(idx) = ranges.binary_search_by(|range| {
            contains_cmp(range, beacon)
        }) else {
            continue;
        };
        let range_to_split = ranges[idx].clone();
        let l = range_to_split.start..idx as i32;
        let r = idx as i32 + 1..range_to_split.end;
        ranges[idx] = l;
        ranges.insert(idx + 1, r);
    }
    ranges.into_iter().map(|range| range.len()).sum()
}

pub fn part1(input: &str) -> usize {
    non_beacons_at_y::<2_000_000>(input)
}

pub fn part2(input: &str) -> usize {
    let (_, _reports) = parse::reports(input).unwrap();
    0
}
