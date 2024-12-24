use itertools::Itertools;

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
            for c in "XMAS".chars() {
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
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn check(a: char, b: char) -> bool {
        a == 'M' && b == 'S' || a == 'S' && b == 'M'
    }

    (1..grid.len() - 1)
        .cartesian_product(1..grid[0].len() - 1)
        .filter(|&(i, k)| {
            grid[i][k] == 'A'
                && check(grid[i - 1][k - 1], grid[i + 1][k + 1])
                && check(grid[i + 1][k - 1], grid[i - 1][k + 1])
        })
        .count()
}
