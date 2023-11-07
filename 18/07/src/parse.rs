use utils::nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, newline},
    multi::separated_list0,
    IResult,
};

use crate::Instruction;

fn step(input: &str) -> IResult<&str, char> {
    let (input, _) = alt((tag("Step "), tag("step ")))(input)?;
    anychar(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, dependency) = step(input)?;
    let (input, _) = tag(" must be finished before ")(input)?;
    let (input, target) = step(input)?;
    let (input, _) = tag(" can begin.")(input)?;
    Ok((input, Instruction { target, dependency }))
}

pub(crate) fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(newline, instruction)(input)
}
