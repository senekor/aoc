use std::cmp::max;
use std::vec::Vec;

struct Range {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

enum Command {
    TurnOn(Range),
    TurnOff(Range),
    Toggle(Range),
}

fn parse_ranges<'a>(words: &mut impl Iterator<Item = &'a str>) -> Range {
    let mut first_xy = words.next().unwrap().split(',');
    words.next(); // consume "through"
    let mut second_xy = words.next().unwrap().split(',');
    let x1: usize = first_xy.next().unwrap().parse().unwrap();
    let y1: usize = first_xy.next().unwrap().parse().unwrap();
    let x2: usize = second_xy.next().unwrap().parse().unwrap();
    let y2: usize = second_xy.next().unwrap().parse().unwrap();
    Range { x1, x2, y1, y2 }
}

fn parse_line(line: &str) -> Command {
    let mut words = line.split(' ');
    if words.next().unwrap() == "toggle" {
        return Command::Toggle(parse_ranges(&mut words));
    }
    if words.next().unwrap() == "on" {
        return Command::TurnOn(parse_ranges(&mut words));
    }
    Command::TurnOff(parse_ranges(&mut words))
}

type Grid<T> = Vec<Vec<T>>;

fn do_for_range<T>(grid: &mut Grid<T>, range: Range, f: fn(&T) -> T) {
    for row in grid.iter_mut().take(range.x2 + 1).skip(range.x1) {
        for cell in row.iter_mut().take(range.y2 + 1).skip(range.y1) {
            *cell = f(cell);
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut one_row = Vec::new();
    one_row.resize(1_000, false);
    let mut grid: Grid<bool> = Vec::new();
    grid.resize(1_000, one_row);
    for line in input.split('\n') {
        match parse_line(line) {
            Command::TurnOn(range) => do_for_range(&mut grid, range, |_| true),
            Command::TurnOff(range) => do_for_range(&mut grid, range, |_| false),
            Command::Toggle(range) => do_for_range(&mut grid, range, |b| !b),
        }
    }

    let mut num_lit_lights = 0;
    for row in grid {
        for light in row {
            if light {
                num_lit_lights += 1;
            }
        }
    }
    num_lit_lights
}

pub fn part2(input: &str) -> i32 {
    let mut grid: Grid<i32> = vec![vec![0; 1_000]; 1_000];
    for line in input.split('\n') {
        match parse_line(line) {
            Command::TurnOn(range) => do_for_range(&mut grid, range, |i| i + 1),
            Command::TurnOff(range) => do_for_range(&mut grid, range, |i| max(0, i - 1)),
            Command::Toggle(range) => do_for_range(&mut grid, range, |i| i + 2),
        }
    }

    let mut total_brightness = 0;
    for row in grid {
        for light in row {
            total_brightness += light;
        }
    }
    total_brightness
}
