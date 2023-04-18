type Cube = Vec<Vec<Vec<bool>>>;

fn lines_to_lists(lines: Vec<&str>, num_cycles: usize) -> Cube {
    let space = 2 * num_cycles;
    let (k, l, m) = (lines.len() + space, lines[0].len() + space, 1 + space);
    let mut init_state = vec![vec![vec![false; m]; l]; k];
    for row_idx in 0..lines.len() {
        for (col_idx, ch) in lines[row_idx].chars().enumerate() {
            init_state[row_idx][col_idx][0] = ch == '#'
        }
    }
    init_state
}

fn count_neighbors(state: &Cube, k: usize, l: usize, m: usize) -> usize {
    let (rows, cols, planes) = (state.len(), state[0].len(), state[0][0].len());
    let mut num_active_neighbors = 0;
    for row in k + rows - 1..k + rows + 2 {
        for col in l + cols - 1..l + cols + 2 {
            for plane in m + planes - 1..m + planes + 2 {
                if state[row % rows][col % cols][plane % planes] {
                    num_active_neighbors += 1
                }
            }
        }
    }
    if state[k][l][m] {
        num_active_neighbors -= 1;
    }
    num_active_neighbors
}

fn calc_activity(state: &Cube, k: usize, l: usize, m: usize) -> bool {
    let num_neighbors = count_neighbors(state, k, l, m);
    state[k][l][m] && (2..=3).contains(&num_neighbors) || num_neighbors == 3
}

fn sim_one_cycle(state: Cube) -> Cube {
    let mut new_state = state.clone();
    for (row_idx, row) in new_state.iter_mut().enumerate() {
        for (col_idx, col) in row.iter_mut().enumerate() {
            for (plane_idx, plane) in col.iter_mut().enumerate() {
                *plane = calc_activity(&state, row_idx, col_idx, plane_idx)
            }
        }
    }
    new_state
}

fn sim_cycles(state: Cube, num_cycles: usize) -> Cube {
    (0..num_cycles).fold(state, |s, _| sim_one_cycle(s))
}

fn count_active(state: Cube) -> usize {
    state.into_iter().flatten().flatten().filter(|&b| b).count()
}

pub fn part1(input: &str) -> usize {
    let num_cycles = 6;
    let init_state = lines_to_lists(input.lines().collect(), num_cycles);
    let end_state = sim_cycles(init_state, num_cycles);
    count_active(end_state)
}

type Hypercube = Vec<Vec<Vec<Vec<bool>>>>;

fn lines_to_lists_v2(lines: Vec<&str>, num_cycles: usize) -> Hypercube {
    let space = 2 * num_cycles;
    let (k, l, m_n) = (lines.len() + space, lines[0].len() + space, 1 + space);
    let mut init_state = vec![vec![vec![vec![false; m_n]; m_n]; l]; k];
    for row_idx in 0..lines.len() {
        for (col_idx, ch) in lines[row_idx].chars().enumerate() {
            init_state[row_idx][col_idx][0][0] = ch == '#'
        }
    }
    init_state
}

fn count_neighbors_v2(state: &Hypercube, k: usize, l: usize, m: usize, n: usize) -> usize {
    let (rows, cols, planes, bruhs) = (
        state.len(),
        state[0].len(),
        state[0][0].len(),
        state[0][0][0].len(),
    );
    let mut num_active_neighbors = 0;
    for row in k + rows - 1..k + rows + 2 {
        for col in l + cols - 1..l + cols + 2 {
            for plane in m + planes - 1..m + planes + 2 {
                for bruh in n + bruhs - 1..n + bruhs + 2 {
                    if state[row % rows][col % cols][plane % planes][bruh % bruhs] {
                        num_active_neighbors += 1
                    }
                }
            }
        }
    }
    if state[k][l][m][n] {
        num_active_neighbors -= 1;
    }
    num_active_neighbors
}

fn calc_activity_v2(state: &Hypercube, k: usize, l: usize, m: usize, n: usize) -> bool {
    let num_neighbors = count_neighbors_v2(state, k, l, m, n);
    state[k][l][m][n] && (2..=3).contains(&num_neighbors) || num_neighbors == 3
}

fn sim_one_cycle_v2(state: Hypercube) -> Hypercube {
    let mut new_state = state.clone();
    for (row_idx, row) in new_state.iter_mut().enumerate() {
        for (col_idx, col) in row.iter_mut().enumerate() {
            for (plane_idx, plane) in col.iter_mut().enumerate() {
                for (bruh_idx, bruh) in plane.iter_mut().enumerate() {
                    *bruh = calc_activity_v2(&state, row_idx, col_idx, plane_idx, bruh_idx)
                }
            }
        }
    }
    new_state
}

fn sim_cycles_v2(state: Hypercube, num_cycles: usize) -> Hypercube {
    (0..num_cycles).fold(state, |s, _| sim_one_cycle_v2(s))
}

fn count_active_v2(state: Hypercube) -> usize {
    state
        .into_iter()
        .flatten()
        .flatten()
        .flatten()
        .filter(|&b| b)
        .count()
}

pub fn part2(input: &str) -> usize {
    let num_cycles = 6;
    let init_state = lines_to_lists_v2(input.lines().collect(), num_cycles);
    let end_state = sim_cycles_v2(init_state, num_cycles);
    count_active_v2(end_state)
}
