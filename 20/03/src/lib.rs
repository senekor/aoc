struct Position {
    x: usize,
    y: usize,
}

fn get(input: &str, pos: &Position) -> char {
    input
        .lines()
        .nth(pos.y)
        .unwrap()
        .chars()
        .nth(pos.x)
        .unwrap()
}

fn calc_trees(input: &str, right: usize, down: usize) -> u32 {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut count = 0;
    let mut pos = Position { x: 0, y: 0 };
    while pos.y < height {
        if get(input, &pos) == '#' {
            count += 1;
        }

        pos.x += right;
        pos.x %= width;
        pos.y += down;
    }

    count
}

pub fn part1(input: &str) -> u32 {
    calc_trees(input, 3, 1)
}

pub fn part2(input: &str) -> u32 {
    calc_trees(input, 1, 1)
        * calc_trees(input, 3, 1)
        * calc_trees(input, 5, 1)
        * calc_trees(input, 7, 1)
        * calc_trees(input, 1, 2)
}
