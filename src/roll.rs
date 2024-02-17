mod parse;
mod realize;
mod value;

use super::Value;
use parse::ParseError;
use rand::Rng;
use realize::RealizeError;
use std::fmt::{self, Write as _};

pub fn main(state: &mut super::State<impl Rng>) -> Option<Result<Value, RollError>> {
    let raw = state.lexer.matching(super::Token::Roll)?;
    Some(inner(raw, &mut state.output, state.rng))
}

fn inner(raw: &str, output: &mut String, rng: &mut impl Rng) -> Result<Value, RollError> {
    let parse = parse::main(raw)?;
    let realize = realize::main(parse, rng)?;
    write!(output, "{realize}")?;
    Ok(value::main(realize))
}

#[derive(Debug)]
pub enum RollError {
    Parse(ParseError),
    Realize(RealizeError),
}

impl From<RollError> for super::EvalError {
    fn from(error: RollError) -> Self {
        Self::Roll(error)
    }
}

impl From<ParseError> for RollError {
    fn from(error: ParseError) -> Self {
        Self::Parse(error)
    }
}

impl From<RealizeError> for RollError {
    fn from(error: RealizeError) -> Self {
        Self::Realize(error)
    }
}

impl From<fmt::Error> for RollError {
    fn from(_: fmt::Error) -> Self {
        unreachable!("formatting should be infallible")
    }
}

impl fmt::Display for RollError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => write!(f, "parsing roll: {error}"),
            Self::Realize(error) => write!(f, "generating roll: {error}"),
        }
    }
}
