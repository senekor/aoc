use std::collections::VecDeque;

use utils::winnow::{
    ascii::{digit1, multispace0, multispace1, space0},
    combinator::{alt, preceded, separated, separated_pair, terminated},
    PResult, Parser,
};

use crate::{Item, Monkey, MonkeyID, Operation};

fn item(input: &mut &str) -> PResult<Item> {
    digit1
        .map(|digits: &str| digits.parse().unwrap())
        .parse_next(input)
}

fn items(input: &mut &str) -> PResult<VecDeque<Item>> {
    let several_items = separated(1.., item, ", ");
    preceded("Starting items: ", several_items)
        .parse_next(input)
        .map(Vec::into)
}

#[test]
fn test_items() {
    assert_eq!(
        items(&mut "Starting items: 79, 98"),
        Ok(vec![79, 98].into())
    );
}

type Operator = fn(Item, Item) -> Item;

fn operator(input: &mut &str) -> PResult<Operator> {
    let plus = '+'.map(|_| std::ops::Add::add as Operator);
    let star = '*'.map(|_| std::ops::Mul::mul as Operator);
    alt((plus, star)).parse_next(input)
}

fn operand(input: &mut &str) -> PResult<Option<Item>> {
    let old = "old".map(|_| None);
    let num = digit1.map(|digits: &str| Some(digits.parse().unwrap()));
    alt((old, num)).parse_next(input)
}

fn operation(input: &mut &str) -> PResult<Operation> {
    let expression = separated_pair(operand, space0, separated_pair(operator, space0, operand));
    preceded("Operation: new = ", expression)
        .map(|(left, (f, right))| (f, left, right))
        .parse_next(input)
}

#[test]
fn test_operation() {
    let op = operation(&mut "Operation: new = old * 19").unwrap();
    assert_eq!(op, (std::ops::Mul::mul as Operator, None, Some(19)));
}

fn divisible(input: &mut &str) -> PResult<Item> {
    preceded("Test: divisible by ", item).parse_next(input)
}

#[test]
fn test_divisible() {
    let divisible = divisible(&mut "Test: divisible by 17").unwrap();
    assert_eq!(divisible, 17);
}

fn monkey_id(input: &mut &str) -> PResult<MonkeyID> {
    digit1
        .map(|digits: &str| digits.parse::<MonkeyID>().unwrap())
        .parse_next(input)
}

fn receiver_true(input: &mut &str) -> PResult<MonkeyID> {
    preceded("If true: throw to monkey ", monkey_id).parse_next(input)
}

fn receiver_false(input: &mut &str) -> PResult<MonkeyID> {
    preceded("If false: throw to monkey ", monkey_id).parse_next(input)
}

fn monkey(input: &mut &str) -> PResult<Monkey> {
    ("Monkey ", digit1, ':').parse_next(input)?;
    let items = preceded(multispace1, items).parse_next(input)?;
    let operation = preceded(multispace1, operation).parse_next(input)?;
    let divisible = preceded(multispace1, divisible).parse_next(input)?;
    let receiver_true = preceded(multispace1, receiver_true).parse_next(input)?;
    let receiver_false = preceded(multispace1, receiver_false).parse_next(input)?;
    Ok(Monkey {
        items,
        operation,
        divisible_by: divisible,
        receiver_true,
        receiver_false,
    })
}

#[test]
fn test_monkey() {
    let monkey = monkey(
        &mut "\
Monkey 4:
  Starting items: 55, 52, 67, 70, 69, 94, 90
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 3",
    )
    .unwrap();
    assert_eq!(monkey.items, vec![55, 52, 67, 70, 69, 94, 90]);
    assert_eq!(
        monkey.operation,
        (std::ops::Mul::mul as Operator, None, None)
    );
    assert_eq!(monkey.divisible_by, 17);
}

pub(crate) fn monkeys(input: &mut &str) -> PResult<Vec<Monkey>> {
    terminated(separated(.., monkey, multispace1), multispace0).parse_next(input)
}
