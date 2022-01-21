use itertools::*;

fn main() {
    let input = include_str!("../input/input.txt");
    let input = input.lines().map(str::to_string).collect_vec();

    part1(input.clone());

    part2(input.clone());
}

fn part1(input: Vec<String>) {
    let mut count = 0;

    for line in input {
        let os = line.split(" | ").nth(1).unwrap();

        for o in os.split(" ") {
            match o.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => {}
            }
        }
    }

    println!("{:?}", count)
}

fn overlap(s1: &str, s2: &str) -> usize {
    let mut count = 0;
    for a in s1.chars() {
        if s2.contains(a) {
            count += 1;
        }
    }
    count
}

fn part2(input: Vec<String>) {
    let mut count = 0;

    for line in input {
        // let os = line.split(" | ").nth(1).unwrap();
        let mut bruh = line.split(" | ");
        let yeet = bruh.join(" ");
        let vec = yeet.split(" ").collect_vec();

        // let mut one = "".to_string();
        let mut four = "".to_string();
        let mut seven = "".to_string();
        // let mut eight = "".to_string();

        for &o in vec.iter() {
            match o.len() {
                // 2 => one = o.to_owned(),
                3 => seven = o.to_owned(),
                4 => four = o.to_owned(),
                // 7 => eight = o.to_owned(),
                _ => {}
            }
        }

        let mut two = "".to_string();
        let mut three = "".to_string();
        // let mut five = "".to_string();
        let mut six = "".to_string();
        // let mut nine = "".to_string();
        let mut zero = "".to_string();
        for &o in vec.iter() {
            if o.len() == 5 {
                if overlap(o, &four) == 2 {
                    two = o.to_owned();
                } else if overlap(o, &seven) == 3 {
                    three = o.to_owned();
                }
                // } else {
                //     five = o.to_owned();
                // }
            }
            if o.len() == 6 {
                if overlap(o, &seven) == 2 {
                    six = o.to_owned();
                } else if overlap(o, &four) == 4 {
                    // nine = o.to_owned();
                } else {
                    zero = o.to_owned();
                }
            }
        }

        let ten: i32 = 10;
        let mut num = 0;
        for (i, &o) in vec[10..vec.len()].iter().rev().enumerate() {
            if o.len() == 2 {
                num += 1 * ten.pow(i as u32);
            } else if o.len() == 3 {
                num += 7 * ten.pow(i as u32);
            } else if o.len() == 4 {
                num += 4 * ten.pow(i as u32);
            } else if o.len() == 7 {
                num += 8 * ten.pow(i as u32);
            } else if o.len() == 5 {
                if overlap(o, &two) == 5 {
                    num += 2 * ten.pow(i as u32);
                } else if overlap(o, &three) == 5 {
                    num += 3 * ten.pow(i as u32);
                } else {
                    num += 5 * ten.pow(i as u32);
                }
            } else if o.len() == 6 {
                if overlap(o, &zero) == 6 {
                } else if overlap(o, &six) == 6 {
                    num += 6 * ten.pow(i as u32);
                } else {
                    num += 9 * ten.pow(i as u32);
                }
            }
        }
        count += num;
    }

    println!("{:?}", count)
}
