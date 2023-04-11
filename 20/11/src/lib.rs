fn neighbors(i: usize, j: usize, seats: &[Vec<u8>]) -> usize {
    (i.saturating_sub(1)..(i + 2).min(seats.len()))
        .flat_map(|ii| (j.saturating_sub(1)..(j + 2).min(seats[0].len())).map(move |jj| (ii, jj)))
        .filter(|&(ii, jj)| !(i == ii && j == jj) && seats[ii][jj] == b'#')
        .count()
}

pub fn part1(input: &str) -> usize {
    let mut seats: Vec<_> = input.lines().map(|row| row.as_bytes().to_vec()).collect();
    let mut new_seats: Vec<_> = seats.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for (i, j) in (0..seats.len()).flat_map(|i| (0..seats[0].len()).map(move |j| (i, j))) {
            let neighbors = neighbors(i, j, &seats);
            new_seats[i][j] = match () {
                _ if seats[i][j] == b'L' && neighbors == 0 => b'#',
                _ if seats[i][j] == b'#' && neighbors >= 4 => b'L',
                _ => seats[i][j],
            };
            changed = changed || new_seats[i][j] != seats[i][j];
        }
        std::mem::swap(&mut seats, &mut new_seats);
    }
    seats
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|&space| space == b'#')
        .count()
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
