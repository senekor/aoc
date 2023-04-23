use std::collections::VecDeque;

use utils::Itertools;

mod parse;

type Item = i64;
type Operation = (fn(Item, Item) -> Item, Option<Item>, Option<Item>);
type MonkeyID = usize;

struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    divisible_by: Item,
    receiver_true: MonkeyID,
    receiver_false: MonkeyID,
}

fn monkey_business(
    input: &str,
    inspect: fn(item: Item, lcm: Item) -> Item,
    rounds: usize,
) -> usize {
    let (_, mut monkeys) = parse::monkeys(input).unwrap();
    let mut activity = vec![0; monkeys.len()];

    let lcm = monkeys
        .iter()
        .map(|m| m.divisible_by)
        .reduce(num::integer::lcm)
        .unwrap();

    for (_round, cur_monkey) in (0..rounds).cartesian_product(0..monkeys.len()) {
        while let Some(item) = monkeys[cur_monkey].items.pop_front() {
            let op = monkeys[cur_monkey].operation;
            let item = (op.0)(op.1.unwrap_or(item), op.2.unwrap_or(item));
            let item = inspect(item, lcm);
            let receiver = if item % monkeys[cur_monkey].divisible_by == 0 {
                monkeys[cur_monkey].receiver_true
            } else {
                monkeys[cur_monkey].receiver_false
            };
            monkeys[receiver].items.push_back(item);

            activity[cur_monkey] += 1;
        }
    }
    activity.sort_by_key(|&a| std::cmp::Reverse(a));
    activity[0] * activity[1]
}

pub fn part1(input: &str) -> usize {
    monkey_business(input, |item, _| item / 3, 20)
}

pub fn part2(input: &str) -> usize {
    monkey_business(input, |item, lcm| item % lcm, 10_000)
}
