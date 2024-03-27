use winnow::{
    ascii::newline,
    combinator::{alt, separated},
    token::any,
    PResult, Parser,
};

use crate::Instruction;

fn step(input: &mut &str) -> PResult<char> {
    alt(("Step ", "step ")).parse_next(input)?;
    any(input)
}

fn instruction(input: &mut &str) -> PResult<Instruction> {
    let dependency = step.parse_next(input)?;
    let _ = " must be finished before ".parse_next(input)?;
    let target = step.parse_next(input)?;
    let _ = " can begin.".parse_next(input)?;
    Ok(Instruction { target, dependency })
}

pub(crate) fn instructions(input: &mut &str) -> PResult<Vec<Instruction>> {
    separated(.., instruction, newline).parse_next(input)
}
