fn top_left(i: usize, j: usize, seats: &[Vec<char>]) -> char {
    if i > 0 && j > 0 {
        seats[i - 1][j - 1]
    } else {
        '.'
    }
}

fn top_mid(i: usize, j: usize, seats: &[Vec<char>]) -> char {
    if i > 0 {
        seats[i - 1][j]
    } else {
        '.'
    }
}

fn top_right(i: usize, j: usize, seats: &[Vec<char>]) -> char {
    if i > 0 && j < seats[0].len() - 1 {
        seats[i - 1][j + 1]
    } else {
        '.'
    }
}

fn left(i: usize, j: usize, seats: &[Vec<char>]) -> char {
    if j > 0 {
        seats[i][j - 1]
    } else {
        '.'
    }
}

fn right(i: usize, j: usize, seats: &[Vec<char>]) -> char {
    if j < seats[0].len() - 1 {
        seats[i][j + 1]
    } else {
        '.'
    }
}

fn bot_left(i: usize, j: usize, seats: &[Vec<char>]) -> char {
    if i < seats.len() - 1 && j > 0 {
        seats[i + 1][j - 1]
    } else {
        '.'
    }
}

fn bot_mid(i: usize, j: usize, seats: &[Vec<char>]) -> char {
    if i < seats.len() - 1 {
        seats[i + 1][j]
    } else {
        '.'
    }
}

fn bot_right(i: usize, j: usize, seats: &[Vec<char>]) -> char {
    if i < seats.len() - 1 && j < seats[0].len() - 1 {
        seats[i + 1][j + 1]
    } else {
        '.'
    }
}

fn neighbors(i: usize, j: usize, seats: &[Vec<char>]) -> usize {
    let mut sum = 0;
    if top_left(i, j, seats) == '#' {
        sum += 1
    }
    if top_mid(i, j, seats) == '#' {
        sum += 1
    }
    if top_right(i, j, seats) == '#' {
        sum += 1
    }
    if left(i, j, seats) == '#' {
        sum += 1
    }
    if right(i, j, seats) == '#' {
        sum += 1
    }
    if bot_left(i, j, seats) == '#' {
        sum += 1
    }
    if bot_mid(i, j, seats) == '#' {
        sum += 1
    }
    if bot_right(i, j, seats) == '#' {
        sum += 1
    }
    sum
}

fn will_be_filled(i: usize, j: usize, seats: &[Vec<char>]) -> bool {
    if seats[i][j] != 'L' {
        return false;
    }
    neighbors(i, j, seats) == 0
}

fn will_be_vacated(i: usize, j: usize, seats: &[Vec<char>]) -> bool {
    if seats[i][j] != '#' {
        return false;
    }
    neighbors(i, j, seats) >= 4
}

pub fn part1(input: &str) -> usize {
    let mut seats: Vec<Vec<_>> = input.lines().map(|row| row.chars().collect()).collect();

    loop {
        let mut changed = false;
        let mut new_seats: Vec<Vec<_>> = seats.clone();
        for i in 0..seats.len() {
            for j in 0..seats[0].len() {
                if seats[i][j] == 'L' && will_be_filled(i, j, &seats) {
                    new_seats[i][j] = '#';
                    changed = true
                } else if seats[i][j] == '#' && will_be_vacated(i, j, &seats) {
                    new_seats[i][j] = 'L';
                    changed = true
                }
            }
        }
        if !changed {
            let mut sum = 0;
            for row in seats {
                for space in row {
                    if space == '#' {
                        sum += 1;
                    }
                }
            }
            return sum;
        }
        seats = new_seats
    }
}

type Seats<'a> = &'a [Vec<(char, Vec<(usize, usize)>)>];

fn check_direction(
    y_axis: i32,
    x_axis: i32,
    i: usize,
    j: usize,
    seats: Seats,
) -> Option<(usize, usize)> {
    let mut i = i as i32;
    let mut j = j as i32;
    loop {
        i += y_axis;
        j += x_axis;
        if i < 0 || i == seats.len() as i32 || j < 0 || j == seats[0].len() as i32 {
            return None;
        }
        let i = i as usize;
        let j = j as usize;
        if seats[i][j].0 == '.' {
            continue;
        }
        return Some((i, j));
    }
}

fn get_visible(seats: Seats, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        check_direction(-1, -1, i, j, seats),
        check_direction(-1, 0, i, j, seats),
        check_direction(-1, 1, i, j, seats),
        check_direction(0, -1, i, j, seats),
        check_direction(0, 1, i, j, seats),
        check_direction(1, -1, i, j, seats),
        check_direction(1, 0, i, j, seats),
        check_direction(1, 1, i, j, seats),
    ]
    .into_iter()
    .flatten()
}

fn visible_people(i: usize, j: usize, seats: Seats) -> usize {
    let mut sum = 0;
    for &(ii, jj) in &seats[i][j].1 {
        if seats[ii][jj].0 == '#' {
            sum += 1;
        }
    }
    sum
}

fn will_be_filled2(i: usize, j: usize, seats: Seats) -> bool {
    visible_people(i, j, seats) == 0
}

fn will_be_vacated2(i: usize, j: usize, seats: Seats) -> bool {
    visible_people(i, j, seats) >= 5
}

pub fn part2(input: &str) -> usize {
    let mut seats: Vec<Vec<_>> = input
        .lines()
        .map(|row| row.chars().map(|c| (c, vec![])).collect())
        .collect();

    for i in 0..seats.len() {
        for j in 0..seats[0].len() {
            let iter = get_visible(&seats, i, j);
            seats[i][j].1.extend(iter)
        }
    }
    loop {
        let mut changed = false;
        let mut new_seats: Vec<Vec<_>> = seats.clone();
        for i in 0..seats.len() {
            for j in 0..seats[0].len() {
                if seats[i][j].0 == 'L' && will_be_filled2(i, j, &seats) {
                    new_seats[i][j].0 = '#';
                    changed = true
                } else if seats[i][j].0 == '#' && will_be_vacated2(i, j, &seats) {
                    new_seats[i][j].0 = 'L';
                    changed = true
                }
            }
        }
        if !changed {
            let mut sum = 0;
            for row in seats {
                for space in row {
                    if space.0 == '#' {
                        sum += 1;
                    }
                }
            }
            return sum;
        }
        seats = new_seats
    }
}
