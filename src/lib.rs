mod number;
mod roll;
mod utils;
mod value;

use logos::Logos;
use rand::Rng;
use std::{
    convert::Infallible,
    fmt::{self, Write as _},
};
use value::Value;

type PartialResult = Result<Value, EvalError>;

// todo implement passing constraints to constrain behavior when evaluating untrusted user input
pub fn eval(raw: &str, rng: &mut impl Rng) -> Result<String, EvalError> {
    let mut state = State {
        lexer: Lexer::new(raw),
        output: String::new(),
        rng,
    };
    let value = expression(&mut state)?;
    let State {
        lexer, mut output, ..
    } = state;
    if !lexer.is_eos() {
        return Err(EvalError::UnexpectedToken);
    }
    write!(output, " = {value}")?;
    Ok(output)
}

#[derive(Debug)]
struct State<'a, 'b, R> {
    lexer: Lexer<'a>,
    output: String,
    rng: &'b mut R,
}

#[derive(Debug)]
pub enum EvalError {
    UnexpectedToken,
    Roll(roll::RollError),
    Value(value::ValueError),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken => write!(f, "malformed expression"),
            Self::Roll(error) => write!(f, "parsing roll: {error}"),
            Self::Value(error) => write!(f, "evaluating expression: {error}"),
        }
    }
}

impl std::error::Error for EvalError {}

impl From<Infallible> for EvalError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}

impl From<fmt::Error> for EvalError {
    fn from(_: fmt::Error) -> Self {
        unreachable!("formatting should be infallible")
    }
}

fn expression(state: &mut State<impl Rng>) -> PartialResult {
    equality(state)
}

macro_rules! binary_rule {
    ($name:ident -> $next:ident { $($token:ident => $func:tt, $repr:literal,)* }) => {
        fn $name(state: &mut State<impl Rng>) -> PartialResult {
            enum Op { $($token),* }
            let mut left = $next(state)?;
            loop {
                // this first branch will be removed by the compiler and it simplifies the macro definition
                let op = if false { unreachable!() }
                $(else if state.lexer.matches(Token::$token) { Op::$token })*
                else { return Ok(left); };
                state.output.push_str(match op {
                    $(Op::$token => $repr),*
                });
                let right = $next(state)?;
                left = match op {
                    $(Op::$token => Value::$func(left, right)?),*
                };
            }
        }
    };
}

macro_rules! unary_rule {
    ($name:ident -> $next:ident { $($token:ident => $func:tt, $repr:literal,)* }) => {
        fn $name(state: &mut State<impl Rng>) -> PartialResult {
            enum Op { $($token),* }
            // this first branch will be removed by the compiler and it simplifies the macro definition
            let op = if false { unreachable!() }
            $(else if state.lexer.matches(Token::$token) { Op::$token })*
            else { return $next(state); };
            state.output.push_str(match op {
                $(Op::$token => $repr),*
            });
            let right = $name(state)?;
            Ok(match op {
                $(Op::$token => Value::$func(right)?),*
            })
        }
    };
}

binary_rule!(equality -> comparison {
    Equals => eq, " = ",
    ExclamationPointEquals => ne, " != ",
});

binary_rule!(comparison -> term {
    LessThan => lt, " < ",
    LessThanEquals => le, " <= ",
    GreaterThan => gt, " > ",
    GreaterThanEquals => ge, " >= ",
});

binary_rule!(term -> factor {
    Plus => add, " + ",
    Minus => sub, " - ",
});

binary_rule!(factor -> unary {
    Asterisk => mul, "*",
    Slant => div, "/",
});

unary_rule!(unary -> primary {
    ExclamationPoint => not, "!",
    Minus => neg, "-",
});

fn primary(state: &mut State<impl Rng>) -> PartialResult {
    if let Some(result) = grouping(state) {
        Ok(result?)
    } else if let Some(result) = number::main(state) {
        Ok(result?)
    } else if let Some(result) = roll::main(state) {
        Ok(result?)
    } else {
        Err(EvalError::UnexpectedToken)
    }
}

fn grouping(state: &mut State<impl Rng>) -> Option<PartialResult> {
    if !state.lexer.matches(Token::OpeningParenthesis) {
        return None;
    }
    state.output.push('(');
    let value = match expression(state) {
        Ok(value) => value,
        Err(error) => return Some(Err(error)),
    };
    state.output.push(')');
    if !state.lexer.matches(Token::ClosingParenthesis) {
        return Some(Err(EvalError::UnexpectedToken));
    }
    Some(Ok(value))
}

#[derive(Debug)]
struct Lexer<'a> {
    next: Option<Result<Token, ()>>,
    source: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        let mut source = Token::lexer(source);
        let next = source.next();
        Lexer { next, source }
    }

    fn is_eos(&self) -> bool {
        self.next.is_none()
    }

    fn advance(&mut self) {
        self.next = self.source.next();
    }

    fn matches(&mut self, token: Token) -> bool {
        let matches = self.next == Some(Ok(token));
        if matches {
            self.advance();
        }
        matches
    }

    fn matching(&mut self, token: Token) -> Option<&str> {
        let slice = self.source.slice();
        self.matches(token).then_some(slice)
    }
}

// https://nvlpubs.nist.gov/nistpubs/Legacy/FIPS/fipspub1-2-1977.pdf
#[derive(Debug, Logos, PartialEq, Eq)]
#[logos(skip r"\s")]
enum Token {
    #[token("!")]
    ExclamationPoint,
    #[token("=")]
    Equals,
    #[token("!=")]
    ExclamationPointEquals,
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessThanEquals,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanEquals,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slant,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("(")]
    OpeningParenthesis,
    #[token(")")]
    ClosingParenthesis,
    #[regex(r"\d+(\.\d*)?")]
    Number,
    #[regex(r"\d*d\d+([a-zA-Z]\d*)*")]
    Roll,
}
