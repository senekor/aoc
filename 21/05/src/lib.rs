use itertools::*;

#[derive(Clone, Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn str_to_line(s: &str) -> Line {
    let mut halves = s.split(" -> ");
    let mut left = halves.next().unwrap().split(',');
    let mut right = halves.next().unwrap().split(',');
    Line {
        x1: left.next().unwrap().parse().unwrap(),
        y1: left.next().unwrap().parse().unwrap(),
        x2: right.next().unwrap().parse().unwrap(),
        y2: right.next().unwrap().parse().unwrap(),
    }
}

#[derive(Clone)]
struct Lines(Vec<Line>);

fn str_to_lines(s: &str) -> Lines {
    Lines(s.lines().map(str_to_line).collect_vec())
}

fn calc_max_coords(lines: &Lines) -> (usize, usize) {
    lines.0.iter().fold((0, 0), |coords, line| {
        let max_x = match true {
            _ if line.x1 > coords.0 && line.x1 > line.x2 => line.x1,
            _ if line.x2 > coords.0 => line.x2,
            _ => coords.0,
        };
        let max_y = match true {
            _ if line.y1 > coords.1 && line.y1 > line.y2 => line.y1,
            _ if line.y2 > coords.1 => line.y2,
            _ => coords.1,
        };
        (max_x, max_y)
    })
}

fn new_grid(lines: &Lines) -> Vec<Vec<i32>> {
    let (max_x, max_y) = calc_max_coords(lines);
    let row = vec![0; max_y + 1];
    vec![row; max_x + 1]
}

pub fn part1(input: &str) -> usize {
    let lines = str_to_lines(input);

    let mut grid = new_grid(&lines);

    for line in lines.0 {
        let (mut x1, mut y1, x2, y2) = (line.x1, line.y1, line.x2, line.y2);
        if x1 != x2 && y1 != y2 {
            continue;
        }
        while x1 != x2 || y1 != y2 {
            grid[x1][y1] += 1;
            if x1 != x2 {
                if x1 > x2 {
                    x1 -= 1;
                } else {
                    x1 += 1;
                }
            } else {
                // y1 != y2
                if y1 > y2 {
                    y1 -= 1;
                } else {
                    y1 += 1;
                }
            }
        }
        grid[x1][y1] += 1;
    }

    let mut num_twos = 0;
    for row in grid {
        for cell in row {
            if cell > 1 {
                num_twos += 1;
            }
        }
    }
    num_twos
}

pub fn part2(input: &str) -> usize {
    let lines = str_to_lines(input);

    let mut grid = new_grid(&lines);

    for line in lines.0 {
        let (mut x1, mut y1, x2, y2) = (line.x1, line.y1, line.x2, line.y2);
        while x1 != x2 || y1 != y2 {
            grid[x1][y1] += 1;
            if x1 != x2 {
                if x1 > x2 {
                    x1 -= 1;
                } else {
                    x1 += 1;
                }
            }
            if y1 != y2 {
                if y1 > y2 {
                    y1 -= 1;
                } else {
                    y1 += 1;
                }
            }
        }
        grid[x1][y1] += 1;
    }

    let mut num_twos = 0;
    for row in grid {
        for cell in row {
            if cell > 1 {
                num_twos += 1;
            }
        }
    }
    num_twos
}
