pub fn part1(input: &str) -> i32 {
    let (mut x, mut y, mut d) = (0, 0, 0);
    for instr in input.lines() {
        let (name, mut val) = (&instr[0..1], instr[1..].parse::<i32>().unwrap());
        if name == "N" || (name == "F" && d == 90) {
            y += val
        } else if name == "S" || (name == "F" && d == 270) {
            y -= val
        } else if name == "E" || (name == "F" && d == 0) {
            x += val
        } else if name == "W" || (name == "F" && d == 180) {
            x -= val
        } else if name == "L" || name == "R" {
            if name == "R" {
                val = 360 - val
            }
            d = (d + val) % 360
        }
    }
    x.abs() + y.abs()
}

pub fn part2(input: &str) -> i32 {
    let (mut x_s, mut y_s) = (0, 0);
    let (mut x_w, mut y_w) = (10, 1);
    for instr in input.lines() {
        let (name, mut val) = (&instr[0..1], instr[1..].parse::<i32>().unwrap());
        if name == "N" {
            y_w += val
        } else if name == "S" {
            y_w -= val
        } else if name == "E" {
            x_w += val
        } else if name == "W" {
            x_w -= val
        } else if name == "F" {
            (x_s, y_s) = (x_s + val * x_w, y_s + val * y_w)
        } else if name == "L" || name == "R" {
            if name == "R" {
                val = 360 - val
            }
            if val == 90 {
                (x_w, y_w) = (-y_w, x_w)
            } else if val == 180 {
                (x_w, y_w) = (-x_w, -y_w)
            } else if val == 270 {
                (x_w, y_w) = (y_w, -x_w)
            }
        }
    }
    x_s.abs() + y_s.abs()
}
