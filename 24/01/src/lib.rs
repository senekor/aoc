pub fn part1(input: &str) -> usize {
    let (mut left_list, mut right_list) = parse_collections::<Vec<_>>(input);
    left_list.sort();
    right_list.sort();
    left_list
        .into_iter()
        .zip(right_list)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (left_bag, right_bag) = parse_collections::<hashbag::HashBag<_>>(input);
    left_bag
        .into_iter()
        .flat_map(|(item, left_count)| {
            right_bag
                .get(&item)
                .map(|(_, right_count)| item * left_count * right_count)
        })
        .sum()
}

fn parse_collections<T: Extend<usize> + Default>(input: &str) -> (T, T) {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            )
        })
        .fold(
            (T::default(), T::default()),
            |(mut left_list, mut right_list), (left, right)| {
                left_list.extend(std::iter::once(left));
                right_list.extend(std::iter::once(right));
                (left_list, right_list)
            },
        )
}
