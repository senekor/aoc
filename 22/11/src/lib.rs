use std::collections::VecDeque;

use itertools::Itertools;

mod parse;

type Item = i32;
type Operation = Box<dyn Fn(Item) -> Item>;
type Test = Box<dyn Fn(Item) -> bool>;
type MonkeyID = usize;

struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    divisible: Test,
    receiver_true: MonkeyID,
    receiver_false: MonkeyID,
}

fn inspect(item: Item) -> Item {
    item / 3
}

pub fn part1(input: &str) -> usize {
    let (_, mut monkeys) = parse::monkeys(input).unwrap();
    let mut activity = vec![0; monkeys.len()];

    for (_round, cur_monkey) in (0..20).cartesian_product(0..monkeys.len()) {
        while let Some(item) = monkeys[cur_monkey].items.pop_front() {
            let item = (monkeys[cur_monkey].operation)(item);
            let item = inspect(item);
            let receiver = if (monkeys[cur_monkey].divisible)(item) {
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

pub fn part2(_input: &str) -> usize {
    0
}
