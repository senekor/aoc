use std::collections::HashSet;

fn is_low_point(input: &[Vec<i32>], i: usize, j: usize) -> bool {
    let max_i = input.len() - 1;
    let max_j = input[0].len() - 1;
    let n1 = if i == 0 { i32::MAX } else { input[i - 1][j] };
    let n2 = if j == 0 { i32::MAX } else { input[i][j - 1] };
    let n3 = if i == max_i {
        i32::MAX
    } else {
        input[i + 1][j]
    };
    let n4 = if j == max_j {
        i32::MAX
    } else {
        input[i][j + 1]
    };

    let elem = input[i][j];

    elem < n1 && elem < n2 && elem < n3 && elem < n4
}

pub fn part1(input: &str) -> i32 {
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| n.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if is_low_point(&input, i, j) {
                sum += 1 + input[i][j]
            }
        }
    }
    sum
}

fn rec_find_basin(
    input: &[Vec<i32>],
    i: usize,
    j: usize,
    basin_locs: &mut HashSet<(usize, usize)>,
) {
    let max_i = input.len() - 1;
    let max_j = input[0].len() - 1;

    if i != 0 && input[i - 1][j] != 9 && !basin_locs.contains(&(i - 1, j)) {
        basin_locs.insert((i - 1, j));
        rec_find_basin(input, i - 1, j, basin_locs);
    };
    if j != 0 && input[i][j - 1] != 9 && !basin_locs.contains(&(i, j - 1)) {
        basin_locs.insert((i, j - 1));
        rec_find_basin(input, i, j - 1, basin_locs);
    };
    if i != max_i && input[i + 1][j] != 9 && !basin_locs.contains(&(i + 1, j)) {
        basin_locs.insert((i + 1, j));
        rec_find_basin(input, i + 1, j, basin_locs);
    };
    if j != max_j && input[i][j + 1] != 9 && !basin_locs.contains(&(i, j + 1)) {
        basin_locs.insert((i, j + 1));
        rec_find_basin(input, i, j + 1, basin_locs);
    };
}

fn calc_basin_size(input: &[Vec<i32>], i: usize, j: usize) -> usize {
    let mut basin_locs = HashSet::new();
    basin_locs.insert((i, j));

    rec_find_basin(input, i, j, &mut basin_locs);

    basin_locs.len()
}

pub fn part2(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| n.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut basins = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if is_low_point(&input, i, j) {
                // println!("{} {} {}", input[i][j], i, j);
                basins.push(calc_basin_size(&input, i, j))
            }
        }
    }
    basins.sort_unstable();
    basins[basins.len() - 3..basins.len()]
        .iter()
        .product::<usize>()
}
