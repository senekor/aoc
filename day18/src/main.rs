use itertools::*;

#[derive(Debug, Clone)]
enum Snail {
    Pair(Box<Snail>, Box<Snail>),
    Regular(i32),
}
use Snail::*;

fn parse_pair(chars: &mut impl Iterator<Item = char>) -> Snail {
    let left = parse_snail(chars);
    assert_eq!(chars.next(), Some(','));
    let right = parse_snail(chars);
    assert_eq!(chars.next(), Some(']'));
    return Pair(Box::new(left), Box::new(right));
}

fn parse_snail(chars: &mut impl Iterator<Item = char>) -> Snail {
    let first = chars.next().unwrap();
    if first == '[' {
        return parse_pair(chars);
    }
    Regular(first.to_string().parse::<i32>().unwrap())
}

fn add_left(snail: &mut Snail, num: i32) {
    match snail {
        Pair(l, _) => add_left(l, num),
        Regular(x) => *x += num,
    }
}

fn add_right(snail: &mut Snail, num: i32) {
    match snail {
        Pair(_, r) => add_right(r, num),
        Regular(x) => *x += num,
    }
}

fn did_explode(snail: &mut Snail, depth: i32) -> Option<(Option<i32>, Option<i32>)> {
    if let Regular(_) = snail {
        return None;
    }
    if let Pair(l, r) = snail {
        if depth == 5 {
            if let Regular(l) = l.as_mut() {
                if let Regular(r) = r.as_mut() {
                    return Some((Some(*l), Some(*r)));
                }
            }
            panic!("at depth 5, only regular numbers must remain");
        }
        if let Some((left_expl, right_expl)) = did_explode(l, depth + 1) {
            if let Some(right_expl) = right_expl {
                if let Some(_) = left_expl {
                    *l = Box::new(Regular(0));
                }
                add_left(r, right_expl);
            }
            return Some((left_expl, None));
        }
        if let Some((left_expl, right_expl)) = did_explode(r, depth + 1) {
            if let Some(left_expl) = left_expl {
                if let Some(_) = right_expl {
                    *r = Box::new(Regular(0));
                }
                add_right(l, left_expl);
            }
            return Some((None, right_expl));
        }
        return None;
    }
    panic!("wtf?")
}

fn split(snail: &mut Snail) -> bool {
    if let Regular(x) = snail {
        if *x >= 10 {
            *snail = Pair(Box::new(Regular(*x / 2)), Box::new(Regular((*x + 1) / 2)));
            return true;
        }
        return false;
    }
    if let Pair(l, r) = snail {
        return split(l) || split(r);
    }
    panic!("wtf?")
}

fn reduce_snail(snail: &mut Snail) {
    loop {
        if let Some(_) = did_explode(snail, 1) {
            continue;
        }
        if split(snail) {
            continue;
        }
        break;
    }
}

fn snail_sum(left: Snail, right: Snail) -> Snail {
    let mut sum = Pair(Box::new(left), Box::new(right));
    reduce_snail(&mut sum);
    return sum;
}

fn calc_magnitude(snail: &Snail) -> i32 {
    match snail {
        Regular(x) => *x,
        Pair(l, r) => 3 * calc_magnitude(l) + 2 * calc_magnitude(r),
    }
}

fn part1(snails: Vec<Snail>) {
    let result = snails.into_iter().reduce(snail_sum).unwrap();
    let magnitude = calc_magnitude(&result);
    println!("{:#?}", magnitude)
}

fn part2(snails: Vec<Snail>) {
    let mut max = i32::MIN;

    for i in 0..snails.len() {
        for j in 0..snails.len() {
            if i == j {
                continue;
            }
            let mag = calc_magnitude(&snail_sum(snails[i].clone(), snails[j].clone()));
            if mag > max {
                max = mag;
            }
        }
    }
    println!("{:?}", max)
}

fn main() {
    let input = include_str!("../input/input.txt");
    let snails = input
        .lines()
        .map(|line| parse_snail(&mut line.chars()))
        .collect_vec();

    part1(snails.clone());

    part2(snails.clone());
}
