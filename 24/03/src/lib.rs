pub fn part1(input: &str) -> usize {
    input
        .split("mul(")
        .skip(1)
        .filter_map(|rest| {
            let (rest, _) = rest.split_once(')')?;
            let (left, right) = rest.split_once(',')?;
            if [left, right].iter().any(|num| num.len() > 3) {
                return None;
            }
            let left = left.parse::<usize>().ok()?;
            let right = right.parse::<usize>().ok()?;
            Some(left * right)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut iter = input.split("don't()");
    let Some(first) = iter.next() else {
        return 0;
    };
    let first = part1(first);

    iter.filter_map(|disabled_section| {
        disabled_section
            .split_once("do()")
            .map(|(_, enabled_section)| part1(enabled_section))
    })
    .chain(std::iter::once(first))
    .sum()
}
