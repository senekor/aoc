use utils::Itertools;

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
    Pair(Box::new(left), Box::new(right))
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
            if let Regular(left) = l.as_mut() {
                if let Regular(right) = r.as_mut() {
                    return Some((Some(*left), Some(*right)));
                }
            }
            panic!("at depth 5, only regular numbers must remain");
        }
        if let Some((maybe_left_expl, maybe_right_expl)) = did_explode(l, depth + 1) {
            if let Some(right_expl) = maybe_right_expl {
                if maybe_left_expl.is_some() {
                    *l = Box::new(Regular(0));
                }
                add_left(r, right_expl);
            }
            return Some((maybe_left_expl, None));
        }
        if let Some((maybe_left_expl, maybe_right_expl)) = did_explode(r, depth + 1) {
            if let Some(left_expl) = maybe_left_expl {
                if maybe_right_expl.is_some() {
                    *r = Box::new(Regular(0));
                }
                add_right(l, left_expl);
            }
            return Some((None, maybe_right_expl));
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
        if did_explode(snail, 1).is_some() {
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
    sum
}

fn calc_magnitude(snail: &Snail) -> i32 {
    match snail {
        Regular(x) => *x,
        Pair(l, r) => 3 * calc_magnitude(l) + 2 * calc_magnitude(r),
    }
}

pub fn part1(input: &str) -> i32 {
    let snails = input
        .lines()
        .map(|line| parse_snail(&mut line.chars()))
        .collect_vec();

    let result = snails.into_iter().reduce(snail_sum).unwrap();
    calc_magnitude(&result)
}

pub fn part2(input: &str) -> i32 {
    let snails = input
        .lines()
        .map(|line| parse_snail(&mut line.chars()))
        .collect_vec();

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
    max
}
