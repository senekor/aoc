use std::collections::VecDeque;

use utils::nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1, space0},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::{Item, Monkey, MonkeyID, Operation};

fn item(input: &str) -> IResult<&str, Item> {
    map(digit1, |digits: &str| digits.parse().unwrap())(input)
}

fn items(input: &str) -> IResult<&str, VecDeque<Item>> {
    let several_items = separated_list1(tag(", "), item);
    map(
        preceded(tag("Starting items: "), several_items),
        VecDeque::from,
    )(input)
}

#[test]
fn test_items() {
    assert_eq!(
        items("Starting items: 79, 98"),
        Ok(("", vec![79, 98].into()))
    );
}

type Operator = fn(Item, Item) -> Item;

fn operator(input: &str) -> IResult<&str, Operator> {
    let plus = map(char('+'), |_| std::ops::Add::add as Operator);
    let star = map(char('*'), |_| std::ops::Mul::mul as Operator);
    alt((plus, star))(input)
}

fn operand(input: &str) -> IResult<&str, Option<Item>> {
    let old = map(tag("old"), |_| None);
    let num = map(digit1, |digits: &str| Some(digits.parse().unwrap()));
    alt((old, num))(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let expression = separated_pair(operand, space0, separated_pair(operator, space0, operand));
    map(
        preceded(tag("Operation: new = "), expression),
        |(left, (f, right))| (f, left, right),
    )(input)
}

#[test]
fn test_operation() {
    let (rest, op) = operation("Operation: new = old * 19").unwrap();
    assert_eq!(rest, "");
    assert_eq!(op, (std::ops::Mul::mul as Operator, None, Some(19)));
}

fn divisible(input: &str) -> IResult<&str, Item> {
    preceded(tag("Test: divisible by "), item)(input)
}

#[test]
fn test_divisible() {
    let (rest, divisible) = divisible("Test: divisible by 17").unwrap();
    assert_eq!(rest, "");
    assert_eq!(divisible, 17);
}

fn monkey_id(input: &str) -> IResult<&str, MonkeyID> {
    map(digit1, |digits: &str| digits.parse::<MonkeyID>().unwrap())(input)
}

fn receiver_true(input: &str) -> IResult<&str, MonkeyID> {
    preceded(tag("If true: throw to monkey "), monkey_id)(input)
}

fn receiver_false(input: &str) -> IResult<&str, MonkeyID> {
    preceded(tag("If false: throw to monkey "), monkey_id)(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tuple((tag("Monkey "), digit1, char(':')))(input)?;
    let (input, items) = preceded(multispace1, items)(input)?;
    let (input, operation) = preceded(multispace1, operation)(input)?;
    let (input, divisible) = preceded(multispace1, divisible)(input)?;
    let (input, receiver_true) = preceded(multispace1, receiver_true)(input)?;
    let (input, receiver_false) = preceded(multispace1, receiver_false)(input)?;
    Ok((
        input,
        Monkey {
            items,
            operation,
            divisible_by: divisible,
            receiver_true,
            receiver_false,
        },
    ))
}

#[test]
fn test_monkey() {
    let (rest, monkey) = monkey(
        "\
Monkey 4:
  Starting items: 55, 52, 67, 70, 69, 94, 90
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 3",
    )
    .unwrap();
    assert_eq!(rest, "");
    assert_eq!(monkey.items, vec![55, 52, 67, 70, 69, 94, 90]);
    assert_eq!(
        monkey.operation,
        (std::ops::Mul::mul as Operator, None, None)
    );
    assert_eq!(monkey.divisible_by, 17);
}

pub(crate) fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    terminated(separated_list0(multispace1, monkey), multispace0)(input)
}
