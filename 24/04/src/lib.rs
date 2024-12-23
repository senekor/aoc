use itertools::Itertools;

const NEEDLE: &str = "XMAS";

pub fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;

    for (dx, dy) in directions {
        'search_space: for (mut i, mut k) in (0..grid.len()).cartesian_product(0..grid[0].len()) {
            let mut out_of_bounds = false;
            for c in NEEDLE.chars() {
                if out_of_bounds || c != grid[i][k] {
                    continue 'search_space;
                }
                match (i.checked_add_signed(dx), k.checked_add_signed(dy)) {
                    (Some(i_new), Some(k_new)) if i_new < grid.len() && k_new < grid[0].len() => {
                        i = i_new;
                        k = k_new;
                    }
                    _ => out_of_bounds = true,
                }
            }
            count += 1;
        }
    }

    count
}

pub fn part2(input: &str) -> usize {
    0
}
