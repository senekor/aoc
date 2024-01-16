use arrayvec::ArrayVec;

pub fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut sum = 0;

    for row_idx in 0..grid.len() {
        let mut num_start_idx = 0;
        while num_start_idx < grid[0].len() {
            if !grid[row_idx][num_start_idx].is_ascii_digit() {
                num_start_idx += 1;
                continue;
            }
            let mut num_end_idx = num_start_idx + 1;
            while num_end_idx < grid[0].len() && grid[row_idx][num_end_idx].is_ascii_digit() {
                num_end_idx += 1;
            }
            let is_surrounded_by_sympol = [
                row_idx > 0
                    && grid[row_idx - 1]
                        [num_start_idx.saturating_sub(1)..grid[0].len().min(num_end_idx + 1)]
                        .iter()
                        .any(|&c| c != b'.'),
                row_idx < grid.len() - 1
                    && grid[row_idx + 1]
                        [num_start_idx.saturating_sub(1)..grid[0].len().min(num_end_idx + 1)]
                        .iter()
                        .any(|&c| c != b'.'),
                num_start_idx > 0 && grid[row_idx][num_start_idx - 1] != b'.',
                num_end_idx < grid[0].len() && grid[row_idx][num_end_idx] != b'.',
            ]
            .iter()
            .any(|&b| b);
            if is_surrounded_by_sympol {
                let num = std::str::from_utf8(&grid[row_idx][num_start_idx..num_end_idx])
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                sum += num;
            }
            num_start_idx = num_end_idx + 1;
        }
    }

    sum
}

pub fn part2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut sum = 0;

    for (star_row, star_col) in (0..grid.len())
        .flat_map(|i| (0..grid[0].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] == b'*')
    {
        if let Some((n, m)) = find_adjacent_numbers(&grid, star_row, star_col) {
            sum += n * m;
        }
    }

    sum
}

fn find_adjacent_numbers(
    grid: &[&[u8]],
    star_row: usize,
    star_col: usize,
) -> Option<(usize, usize)> {
    let mut nums = ArrayVec::<usize, 2>::new();

    if star_row > 0 {
        // has top row
        let row = &grid[star_row - 1];
        if star_col > 0 {
            if star_col < row.len() - 1 {
                // full row
                if row[star_col - 1].is_ascii_digit()
                    && !row[star_col].is_ascii_digit()
                    && row[star_col + 1].is_ascii_digit()
                {
                    // special case. two numbers on one line
                    nums.push(get_number_at(row, star_col - 1));
                    nums.push(get_number_at(row, star_col + 1));
                } else if row[star_col - 1].is_ascii_digit() {
                    nums.push(get_number_at(row, star_col - 1));
                } else if row[star_col].is_ascii_digit() {
                    nums.push(get_number_at(row, star_col));
                } else if row[star_col + 1].is_ascii_digit() {
                    nums.push(get_number_at(row, star_col + 1));
                }
            } else if row[star_col - 1].is_ascii_digit() {
                nums.push(get_number_at(row, star_col - 1));
            } else if row[star_col].is_ascii_digit() {
                nums.push(get_number_at(row, star_col));
            }
        } else if row[star_col].is_ascii_digit() {
            nums.push(get_number_at(row, star_col));
        } else if row[star_col + 1].is_ascii_digit() {
            nums.push(get_number_at(row, star_col + 1));
        }
    }
    if star_col > 0 && grid[star_row][star_col - 1].is_ascii_digit() {
        if nums.is_full() {
            return None;
        }
        nums.push(get_number_at(grid[star_row], star_col - 1));
    }
    if star_col < grid[0].len() - 1 && grid[star_row][star_col + 1].is_ascii_digit() {
        if nums.is_full() {
            return None;
        }
        nums.push(get_number_at(grid[star_row], star_col + 1));
    }
    if star_row < grid.len() - 1 {
        // has bottom row
        let row = &grid[star_row + 1];
        if star_col > 0 {
            if star_col < row.len() - 1 {
                // full row
                if row[star_col - 1].is_ascii_digit()
                    && !row[star_col].is_ascii_digit()
                    && row[star_col + 1].is_ascii_digit()
                {
                    // special case. two numbers on one line
                    if !nums.is_empty() {
                        return None;
                    }
                    nums.push(get_number_at(row, star_col - 1));
                    nums.push(get_number_at(row, star_col + 1));
                } else if row[star_col - 1].is_ascii_digit() {
                    if nums.is_full() {
                        return None;
                    }
                    nums.push(get_number_at(row, star_col - 1));
                } else if row[star_col].is_ascii_digit() {
                    if nums.is_full() {
                        return None;
                    }
                    nums.push(get_number_at(row, star_col));
                } else if row[star_col + 1].is_ascii_digit() {
                    if nums.is_full() {
                        return None;
                    }
                    nums.push(get_number_at(row, star_col + 1));
                }
            } else if row[star_col - 1].is_ascii_digit() {
                if nums.is_full() {
                    return None;
                }
                nums.push(get_number_at(row, star_col - 1));
            } else if row[star_col].is_ascii_digit() {
                if nums.is_full() {
                    return None;
                }
                nums.push(get_number_at(row, star_col));
            }
        } else if row[star_col].is_ascii_digit() {
            if nums.is_full() {
                return None;
            }
            nums.push(get_number_at(row, star_col));
        } else if row[star_col + 1].is_ascii_digit() {
            if nums.is_full() {
                return None;
            }
            nums.push(get_number_at(row, star_col + 1));
        }
    }

    if nums.is_full() {
        Some((nums[0], nums[1]))
    } else {
        None
    }
}

fn get_number_at(row: &[u8], col: usize) -> usize {
    let mut start = col;
    while start > 0 && row[start - 1].is_ascii_digit() {
        start -= 1;
    }
    let mut end = col + 1;
    while end < row.len() && row[end].is_ascii_digit() {
        end += 1;
    }
    std::str::from_utf8(&row[start..end])
        .unwrap()
        .parse()
        .unwrap()
}
