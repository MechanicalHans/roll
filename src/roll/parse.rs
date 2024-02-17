use crate::utils::parse_validated;
use num::BigUint;
use std::fmt;

#[derive(Debug)]
pub struct Parse {
    pub roll_len: BigUint,
    pub die_size: BigUint,
    pub drop_least: Option<BigUint>,
    pub drop_greatest: Option<BigUint>,
    pub count_greater: Option<BigUint>,
    pub count_lesser: Option<BigUint>,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidArgumentKey,
    RepeatedArgumentKey,
    EmptyArgumentValue,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgumentKey => write!(f, "invalid argument key"),
            Self::RepeatedArgumentKey => write!(f, "repeated argument key"),
            Self::EmptyArgumentValue => write!(f, "empty argument value"),
        }
    }
}

pub fn main(raw: &str) -> Result<Parse, ParseError> {
    use num::One as _;
    use std::cell::OnceCell;

    let (roll_len, rest) = raw
        .split_once('d')
        .expect("lexer should validate existence");
    let (die_size, mut rest) = split_once_number(rest);
    let drop_least = OnceCell::new();
    let drop_greatest = OnceCell::new();
    let count_greater = OnceCell::new();
    let count_lesser = OnceCell::new();
    let map = [
        ("dl", &drop_least),
        ("dg", &drop_greatest),
        ("cg", &count_greater),
        ("cl", &count_lesser),
    ];
    'outer: loop {
        if rest.is_empty() {
            break;
        }
        for (key, value) in map {
            if let Some(suffix) = rest.strip_prefix(key) {
                let (arg, tail) = split_once_number(suffix);
                if value.set(arg).is_err() {
                    return Err(ParseError::RepeatedArgumentKey);
                }
                rest = tail;
                continue 'outer;
            }
        }
        return Err(ParseError::InvalidArgumentKey);
    }

    macro_rules! parse_arg {
        ($arg:ident) => {
            match $arg.into_inner() {
                Some("") => return Err(ParseError::EmptyArgumentValue),
                Some(non_empty) => Some(parse_validated(non_empty)),
                None => None,
            }
        };
    }

    let roll_len = if !roll_len.is_empty() {
        parse_validated(roll_len)
    } else {
        BigUint::one()
    };
    Ok(Parse {
        roll_len,
        die_size: parse_validated(die_size),
        drop_least: parse_arg!(drop_least),
        drop_greatest: parse_arg!(drop_greatest),
        count_greater: parse_arg!(count_greater),
        count_lesser: parse_arg!(count_lesser),
    })
}

fn split_once_number(s: &str) -> (&str, &str) {
    let mid = s
        .bytes()
        .position(|byte| !byte.is_ascii_digit())
        .unwrap_or(s.len());
    s.split_at(mid)
}
