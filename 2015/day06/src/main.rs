use std::cmp::max;
use std::str::Split;
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

fn parse_ranges(words: &mut Split<&str>) -> Range {
    let mut first_xy = words.next().unwrap().split(",");
    words.next(); // consume "through"
    let mut second_xy = words.next().unwrap().split(",");
    let x1: usize = first_xy.next().unwrap().parse().unwrap();
    let y1: usize = first_xy.next().unwrap().parse().unwrap();
    let x2: usize = second_xy.next().unwrap().parse().unwrap();
    let y2: usize = second_xy.next().unwrap().parse().unwrap();
    return Range { x1, x2, y1, y2 };
}

fn parse_line(line: &str) -> Command {
    let mut words = line.split(" ");
    if words.next().unwrap() == "toggle" {
        return Command::Toggle(parse_ranges(&mut words));
    }
    if words.next().unwrap() == "on" {
        return Command::TurnOn(parse_ranges(&mut words));
    }
    return Command::TurnOff(parse_ranges(&mut words));
}

type Grid<T> = Vec<Vec<T>>;

fn do_for_range<T>(grid: &mut Grid<T>, range: Range, f: fn(&T) -> T) {
    for x in range.x1..=range.x2 {
        for y in range.y1..=range.y2 {
            grid[x][y] = f(&grid[x][y]);
        }
    }
}

fn part1(input: &str) {
    let mut one_row = Vec::new();
    one_row.resize(1_000, false);
    let mut grid: Grid<bool> = Vec::new();
    grid.resize(1_000, one_row);
    for line in input.split("\n") {
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
    println!("number of lit lights: {}", num_lit_lights);
}

fn part2(input: &str) {
    let mut one_row = Vec::new();
    one_row.resize(1_000, 0);
    let mut grid: Grid<i32> = Vec::new();
    grid.resize(1_000, one_row);
    for line in input.split("\n") {
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
    println!("total brightness: {}", total_brightness);
}

fn main() {
    let input = include_str!("../input/input.txt");

    part1(input);

    part2(input);
}
