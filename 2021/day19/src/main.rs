use std::{collections::HashSet, str::FromStr};

use itertools::*;

#[cfg(test)]
mod test {
    use std::vec;

    use crate::{Coordinates, Scanner};

    #[test]
    fn test_rotations() {
        let scanner = Scanner {
            id: 0,
            beacons: vec![Coordinates { x: 1, y: 2, z: 3 }],
        };
        let rotations = scanner.rotations();
        // one
        assert_eq!(rotations[0].beacons[0], Coordinates { x: 1, y: 2, z: 3 });
        assert_eq!(rotations[1].beacons[0], Coordinates { x: -2, y: 1, z: 3 });
        assert_eq!(rotations[2].beacons[0], Coordinates { x: -1, y: -2, z: 3 });
        assert_eq!(rotations[3].beacons[0], Coordinates { x: 2, y: -1, z: 3 });
        // two
        assert_eq!(rotations[4].beacons[0], Coordinates { x: 3, y: 2, z: -1 });
        assert_eq!(rotations[5].beacons[0], Coordinates { x: -2, y: 3, z: -1 });
        assert_eq!(
            rotations[6].beacons[0],
            Coordinates {
                x: -3,
                y: -2,
                z: -1
            }
        );
        assert_eq!(rotations[7].beacons[0], Coordinates { x: 2, y: -3, z: -1 });
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Coordinates {
    x: i32,
    y: i32,
    z: i32,
}

fn rotate_axis(degrees: Degree, other_axis_1: &mut i32, other_axis_2: &mut i32) {
    let prev_val_other_axis_1 = *other_axis_1;
    match degrees {
        Degree::Zero => {}
        Degree::Ninety => {
            *other_axis_1 = -*other_axis_2;
            *other_axis_2 = prev_val_other_axis_1;
        }
        Degree::OneEighty => {
            *other_axis_1 = -*other_axis_1;
            *other_axis_2 = -*other_axis_2;
        }
        Degree::TwoSeventy => {
            *other_axis_1 = *other_axis_2;
            *other_axis_2 = -prev_val_other_axis_1;
        }
    }
}

impl Coordinates {
    fn add(&self, other: &Self) -> Coordinates {
        Coordinates {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn neg(&self) -> Coordinates {
        Coordinates {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    fn rotate(mut self, rotation: Rotation) -> Coordinates {
        rotate_axis(rotation.x, &mut self.y, &mut self.z);
        rotate_axis(rotation.y, &mut self.z, &mut self.x);
        rotate_axis(rotation.z, &mut self.x, &mut self.y);
        self
    }

    fn manhattan(&self, other: &Coordinates) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }
}

impl FromStr for Coordinates {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',').map(|s| s.parse().unwrap());
        Ok(Coordinates {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
            z: iter.next().unwrap(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Degree {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

impl Default for Degree {
    fn default() -> Self {
        Self::Zero
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct Rotation {
    x: Degree,
    y: Degree,
    z: Degree,
}

impl Rotation {
    fn all() -> Vec<Rotation> {
        vec![
            // one
            Rotation {
                x: Degree::Zero,
                y: Degree::Zero,
                z: Degree::Zero,
            },
            Rotation {
                x: Degree::Zero,
                y: Degree::Zero,
                z: Degree::Ninety,
            },
            Rotation {
                x: Degree::Zero,
                y: Degree::Zero,
                z: Degree::OneEighty,
            },
            Rotation {
                x: Degree::Zero,
                y: Degree::Zero,
                z: Degree::TwoSeventy,
            },
            // two
            Rotation {
                x: Degree::Zero,
                y: Degree::Ninety,
                z: Degree::Zero,
            },
            Rotation {
                x: Degree::Zero,
                y: Degree::Ninety,
                z: Degree::Ninety,
            },
            Rotation {
                x: Degree::Zero,
                y: Degree::Ninety,
                z: Degree::OneEighty,
            },
            Rotation {
                x: Degree::Zero,
                y: Degree::Ninety,
                z: Degree::TwoSeventy,
            },
            // three
            Rotation {
                x: Degree::Zero,
                y: Degree::OneEighty,
                z: Degree::Zero,
            },
            Rotation {
                x: Degree::Zero,
                y: Degree::OneEighty,
                z: Degree::Ninety,
            },
            Rotation {
                x: Degree::Zero,
                y: Degree::OneEighty,
                z: Degree::OneEighty,
            },
            Rotation {
                x: Degree::Zero,
                y: Degree::OneEighty,
                z: Degree::TwoSeventy,
            },
            // four
            Rotation {
                x: Degree::Zero,
                y: Degree::TwoSeventy,
                z: Degree::Zero,
            },
            Rotation {
                x: Degree::Zero,
                y: Degree::TwoSeventy,
                z: Degree::Ninety,
            },
            Rotation {
                x: Degree::Zero,
                y: Degree::TwoSeventy,
                z: Degree::OneEighty,
            },
            Rotation {
                x: Degree::Zero,
                y: Degree::TwoSeventy,
                z: Degree::TwoSeventy,
            },
            // five
            Rotation {
                x: Degree::Ninety,
                y: Degree::Zero,
                z: Degree::Zero,
            },
            Rotation {
                x: Degree::Ninety,
                y: Degree::Zero,
                z: Degree::Ninety,
            },
            Rotation {
                x: Degree::Ninety,
                y: Degree::Zero,
                z: Degree::OneEighty,
            },
            Rotation {
                x: Degree::Ninety,
                y: Degree::Zero,
                z: Degree::TwoSeventy,
            },
            // six
            Rotation {
                x: Degree::TwoSeventy,
                y: Degree::Zero,
                z: Degree::Zero,
            },
            Rotation {
                x: Degree::TwoSeventy,
                y: Degree::Zero,
                z: Degree::Ninety,
            },
            Rotation {
                x: Degree::TwoSeventy,
                y: Degree::Zero,
                z: Degree::OneEighty,
            },
            Rotation {
                x: Degree::TwoSeventy,
                y: Degree::Zero,
                z: Degree::TwoSeventy,
            },
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Scanner {
    id: i32,
    beacons: Vec<Coordinates>,
}

impl Scanner {
    fn rotate(mut self, rotation: Rotation) -> Scanner {
        for beacon in self.beacons.iter_mut() {
            *beacon = beacon.rotate(rotation);
        }
        self
    }

    fn rotations(&self) -> Vec<Scanner> {
        Rotation::all()
            .into_iter()
            .map(|rotation| self.clone().rotate(rotation))
            .collect_vec()
    }
}

impl FromStr for Scanner {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(Scanner {
            id: lines
                .next()
                .unwrap()
                .split(' ')
                .nth(2)
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            beacons: lines.map(|line| line.parse().unwrap()).collect_vec(),
        })
    }
}

#[derive(Debug)]
struct LocatedScanner {
    location: Coordinates,
    beacons: Vec<Coordinates>,
}

impl LocatedScanner {
    fn locate_rotated_scanner(&self, scanner: Scanner) -> Option<LocatedScanner> {
        for anchor_beacon in self.beacons.iter() {
            for assumed_same_beacon in scanner.beacons.iter() {
                let rel_dist = anchor_beacon.add(&assumed_same_beacon.neg());

                let mut overlap = 0;
                for own_beacon in self.beacons.iter() {
                    for other_beacon in scanner.beacons.iter() {
                        if rel_dist == own_beacon.add(&other_beacon.neg()) {
                            overlap += 1;
                            if overlap == 12 {
                                return Some(LocatedScanner {
                                    beacons: scanner.beacons,
                                    location: self.location.add(&rel_dist),
                                });
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn locate_scanner(&self, scanner: &Scanner) -> Option<LocatedScanner> {
        for rotated_scanner in scanner.rotations() {
            if let Some(located_scanner) = self.locate_rotated_scanner(rotated_scanner) {
                return Some(located_scanner);
            }
        }
        None
    }
}

fn locate_all_scanners(mut scanners: Vec<Scanner>) -> Vec<LocatedScanner> {
    let scanner_zero = scanners.swap_remove(0);
    let mut located_scanners = vec![LocatedScanner {
        beacons: scanner_zero.beacons,
        location: Coordinates::default(),
    }];

    while !scanners.is_empty() {
        'outer: for i in 0..scanners.len() {
            for located_scanner in located_scanners.iter() {
                if let Some(newly_located_scanner) = located_scanner.locate_scanner(&scanners[i]) {
                    located_scanners.push(newly_located_scanner);
                    scanners.swap_remove(i);
                    break 'outer;
                }
            }
        }
    }

    located_scanners
}

fn part1(located_scanners: &[LocatedScanner]) {
    let mut unique_beacons = HashSet::new();
    for located_scanner in located_scanners {
        for beacon in located_scanner.beacons.iter() {
            let abs_beacon_location = located_scanner.location.add(beacon);
            unique_beacons.insert(abs_beacon_location);
        }
    }

    println!("{}", unique_beacons.len());
}

fn part2(located_scanners: &[LocatedScanner]) {
    let mut max_manhattan = usize::MIN;

    for a in located_scanners {
        for b in located_scanners {
            max_manhattan = max_manhattan.max(a.location.manhattan(&b.location))
        }
    }

    println!("{}", max_manhattan);
}

fn main() {
    let scanners: Vec<Scanner> = include_str!("../input/input.txt")
        .split("\n\n")
        .map(|block| block.parse().unwrap())
        .collect_vec();

    let located_scanners = locate_all_scanners(scanners);

    part1(&located_scanners);

    part2(&located_scanners);
}
