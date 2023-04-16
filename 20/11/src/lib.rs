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

type Seats<'a> = &'a [Vec<(u8, Vec<(usize, usize)>)>];

fn check_direction(
    y_axis: isize,
    x_axis: isize,
    mut i: usize,
    mut j: usize,
    seats: Seats,
) -> Option<(usize, usize)> {
    loop {
        i = i.checked_add_signed(y_axis)?;
        j = j.checked_add_signed(x_axis)?;
        if seats.get(i)?.get(j)?.0 != b'.' {
            return Some((i, j));
        }
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
    seats[i][j]
        .1
        .iter()
        .filter(|&&(ii, jj)| seats[ii][jj].0 == b'#')
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut seats: Vec<Vec<_>> = input
        .lines()
        .map(|row| row.bytes().map(|b| (b, Vec::with_capacity(8))).collect())
        .collect();
    for i in 0..seats.len() {
        for j in 0..seats[0].len() {
            let iter = get_visible(&seats, i, j);
            seats[i][j].1.extend(iter);
        }
    }
    let mut new_seats = seats.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for (i, j) in (0..seats.len()).flat_map(|i| (0..seats[0].len()).map(move |j| (i, j))) {
            let visible = visible_people(i, j, &seats);
            new_seats[i][j].0 = match () {
                _ if seats[i][j].0 == b'L' && visible == 0 => b'#',
                _ if seats[i][j].0 == b'#' && visible >= 5 => b'L',
                _ => seats[i][j].0,
            };
            changed = changed || new_seats[i][j].0 != seats[i][j].0;
        }
        std::mem::swap(&mut seats, &mut new_seats)
    }
    seats
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|space| space.0 == b'#')
        .count()
}
