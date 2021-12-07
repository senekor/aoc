use itertools::*;

#[macro_export]
macro_rules! parse {
    ( $line:expr, $( $t:ty, $sep:expr ),* ; $lt:ty ) => {{
        let mut rest = $line;
        (
            $({
                let mut iter = rest.split($sep);
                let elem = iter.next().unwrap().parse::<$t>().unwrap();
                rest = iter.next().unwrap();
                elem
            },)*
            rest.parse::<$lt>().unwrap(),
        )
    }};
}

// ---------- adjust these to customize parsing ---------- //
type Line = (i32, i32);
fn parse(line: &str) -> Line {
    parse!(line, i32, "," ; i32)
}
// ------------------------------------------------------- //

fn calc_fuel_consumption(crabs: &Vec<i32>, alignment_pos: i32) -> i32 {
    crabs
        .into_iter()
        .map(|crab| (crab - alignment_pos).abs())
        .sum()
}

fn median_crab(crabs: &Vec<i32>) -> i32 {
    let mut clone = crabs.clone();
    clone.sort();
    clone[clone.len() / 2]
}

fn part1(crabs: Vec<i32>) {
    let mut bruh = median_crab(&crabs);
    loop {
        let prev = calc_fuel_consumption(&crabs, bruh);
        let next = calc_fuel_consumption(&crabs, bruh + 1);
        if next <= prev {
            bruh += 1
        } else {
            break;
        }
    }
    loop {
        let prev = calc_fuel_consumption(&crabs, bruh);
        let next = calc_fuel_consumption(&crabs, bruh - 1);
        if next <= prev {
            bruh -= 1
        } else {
            break;
        }
    }
    println!("{:?}", calc_fuel_consumption(&crabs, bruh))
}

fn calc_fuel_consumption_2(crabs: &Vec<i32>, alignment_pos: i32) -> i32 {
    crabs
        .into_iter()
        .map(|crab| {
            let diff = (crab - alignment_pos).abs();
            if diff > 0 {
                (1..=diff).sum()
            } else {
                0
            }
        })
        .sum()
}

fn part2(crabs: Vec<i32>) {
    let mut bruh = median_crab(&crabs);
    loop {
        let prev = calc_fuel_consumption_2(&crabs, bruh);
        let next = calc_fuel_consumption_2(&crabs, bruh + 1);
        if next <= prev {
            bruh += 1
        } else {
            break;
        }
    }
    loop {
        let prev = calc_fuel_consumption_2(&crabs, bruh);
        let next = calc_fuel_consumption_2(&crabs, bruh - 1);
        if next <= prev {
            bruh -= 1
        } else {
            break;
        }
    }
    println!("{:?}", calc_fuel_consumption_2(&crabs, bruh))
}

fn main() {
    let input = include_str!("../input/input.txt");
    let input = input
        .split(",")
        .map(|line| line.parse::<i32>().unwrap())
        .collect_vec();

    part1(input.clone());

    part2(input.clone());
}