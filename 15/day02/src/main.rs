use std::cmp;
use std::str::FromStr;

fn part1(input: &str) {
    let mut total_size = 0;
    for line in input.split('\n') {
        let mut dims = line.split('x');
        let l = i32::from_str(dims.next().unwrap()).unwrap_or(0);
        let w = i32::from_str(dims.next().unwrap()).unwrap_or(0);
        let h = i32::from_str(dims.next().unwrap()).unwrap_or(0);
        let (x, y, z) = (l * w, l * h, w * h);
        let smallest = cmp::min(cmp::min(x, y), z);
        total_size += 2 * (x + y + z) + smallest;
    }
    println!("total wrapping paper required: {}", total_size);
}

fn part2(input: &str) {
    let mut total_size = 0;
    for line in input.split('\n') {
        let mut dims = line.split('x');
        let l = i32::from_str(dims.next().unwrap()).unwrap_or(0);
        let w = i32::from_str(dims.next().unwrap()).unwrap_or(0);
        let h = i32::from_str(dims.next().unwrap()).unwrap_or(0);
        let mut dim_slice = [l, w, h];
        dim_slice.sort_unstable();
        let circumf = 2 * (dim_slice[0] + dim_slice[1]);
        let volume = dim_slice[0] * dim_slice[1] * dim_slice[2];
        total_size += circumf + volume;
    }
    println!("total ribbon length required: {}", total_size);
}

fn main() {
    let input = include_str!("../input/input.txt").to_string();

    part1(&input);

    part2(&input);
}
