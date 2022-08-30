use itertools::*;

fn flash(input: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    let max_i = input.len() - 1;
    let max_j = input[0].len() - 1;

    if i != 0 {
        let local_i = i - 1;
        if j != 0 {
            let local_j = j - 1;
            if input[local_i][local_j] != 9 {
                input[local_i][local_j] += 1;
                if input[local_i][local_j] == 9 {
                    flash(input, local_i, local_j)
                }
            }
        }
        if j != max_j {
            let local_j = j + 1;
            if input[local_i][local_j] != 9 {
                input[local_i][local_j] += 1;
                if input[local_i][local_j] == 9 {
                    flash(input, local_i, local_j)
                }
            }
        }
        let local_j = j;
        if input[local_i][local_j] != 9 {
            input[local_i][local_j] += 1;
            if input[local_i][local_j] == 9 {
                flash(input, local_i, local_j)
            }
        }
    };
    if i != max_i {
        let local_i = i + 1;
        if j != 0 {
            let local_j = j - 1;
            if input[local_i][local_j] != 9 {
                input[local_i][local_j] += 1;
                if input[local_i][local_j] == 9 {
                    flash(input, local_i, local_j)
                }
            }
        }
        if j != max_j {
            let local_j = j + 1;
            if input[local_i][local_j] != 9 {
                input[local_i][local_j] += 1;
                if input[local_i][local_j] == 9 {
                    flash(input, local_i, local_j)
                }
            }
        }
        let local_j = j;
        if input[local_i][local_j] != 9 {
            input[local_i][local_j] += 1;
            if input[local_i][local_j] == 9 {
                flash(input, local_i, local_j)
            }
        }
    };
    let local_i = i;
    if j != 0 {
        let local_j = j - 1;
        if input[local_i][local_j] != 9 {
            input[local_i][local_j] += 1;
            if input[local_i][local_j] == 9 {
                flash(input, local_i, local_j)
            }
        }
    }
    if j != max_j {
        let local_j = j + 1;
        if input[local_i][local_j] != 9 {
            input[local_i][local_j] += 1;
            if input[local_i][local_j] == 9 {
                flash(input, local_i, local_j)
            }
        }
    }
}

fn part1(mut input: Vec<Vec<i32>>) {
    let mut sum = 0;
    for _ in 0..100 {
        // flash
        let mut next = input.clone();
        for (i, row) in input.iter().enumerate().take(10) {
            for (j, &cell) in row.iter().enumerate().take(10) {
                if cell == 9 {
                    flash(&mut next, i, j)
                }
            }
        }
        input = next;

        // decrement prev flashes
        for row in input.iter_mut().take(10) {
            for cell in row.iter_mut().take(10) {
                if *cell == 9 {
                    *cell = -1;
                    sum += 1;
                }
            }
        }

        // increment by one
        for row in input.iter_mut().take(10) {
            for cell in row.iter_mut().take(10) {
                *cell += 1;
            }
        }
    }

    println!("{:?}", sum)
}

fn part2(mut input: Vec<Vec<i32>>) {
    let mut k = 0;
    loop {
        k += 1;

        // flash
        let mut next = input.clone();
        for (i, row) in input.iter().enumerate().take(10) {
            for (j, &cell) in row.iter().enumerate().take(10) {
                if cell == 9 {
                    flash(&mut next, i, j)
                }
            }
        }
        input = next;

        // check sync
        let mut sync = true;
        for row in input.iter().take(10) {
            for &cell in row.iter().take(10) {
                if cell != 9 {
                    sync = false;
                    break;
                }
            }
        }
        if sync {
            println!("sync: {}", k);
            break;
        }

        // decrement prev flashes
        for row in input.iter_mut().take(10) {
            for cell in row.iter_mut().take(10) {
                if *cell == 9 {
                    *cell = -1;
                }
            }
        }

        // increment by one
        for row in input.iter_mut().take(10) {
            for cell in row.iter_mut().take(10) {
                *cell += 1;
            }
        }
    }
}

fn main() {
    let input = include_str!("../input/input.txt")
        .lines()
        .map(|line| {
            line.split("")
                .filter_map(|c| {
                    if c.is_empty() {
                        None
                    } else {
                        Some(c.parse::<i32>().unwrap())
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    part1(input.clone());

    part2(input);
}
