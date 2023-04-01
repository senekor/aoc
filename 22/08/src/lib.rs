use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let grid = input.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();

    let mut visible = HashSet::new();

    for (i, row) in grid.iter().enumerate() {
        // left-to-right
        let mut max = 0;
        for (j, tree) in row.iter().copied().enumerate() {
            if tree > max {
                visible.insert((i, j));
                max = tree;
            }
        }
        // right-to-left
        let mut max = 0;
        for (j, &tree) in row.iter().enumerate().rev() {
            if tree > max {
                visible.insert((i, j));
                max = tree;
            }
        }
    }
    for j in 0..grid[0].len() {
        // top-to-bottom
        let mut max = 0;
        for (i, row) in grid.iter().enumerate() {
            let tree = row[j];
            if tree > max {
                visible.insert((i, j));
                max = tree;
            }
        }
        // bottom-to-top
        let mut max = 0;
        for (i, row) in grid.iter().enumerate().rev() {
            let tree = row[j];
            if tree > max {
                visible.insert((i, j));
                max = tree;
            }
        }
    }

    visible.len()
}

pub fn part2(input: &str) -> usize {
    let grid = input.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let mut max = 0;
    for (i, j, tree) in grid.iter().enumerate().flat_map(|(i, row)| {
        row.iter()
            .copied()
            .enumerate()
            .map(move |(j, tree)| (i, j, tree))
    }) {
        let mut right = 0;
        for jj in j + 1..grid[0].len() {
            right += 1;
            if grid[i][jj] >= tree {
                break;
            }
        }
        let mut left = 0;
        for jj in (0..j).rev() {
            left += 1;
            if grid[i][jj] >= tree {
                break;
            }
        }
        let mut bottom = 0;
        for row in grid.iter().skip(i + 1) {
            bottom += 1;
            if row[j] >= tree {
                break;
            }
        }
        let mut top = 0;
        for ii in (0..i).rev() {
            top += 1;
            if grid[ii][j] >= tree {
                break;
            }
        }
        let score = right * left * top * bottom;
        if score > max {
            max = score;
        }
    }
    max
}
