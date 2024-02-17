use super::Value;
use crate::utils::parse_validated;
use num::{BigRational, BigUint, One as _};

pub fn main(
    state: &mut super::State<impl rand::Rng>,
) -> Option<Result<Value, std::convert::Infallible>> {
    let raw = state.lexer.matching(super::Token::Number)?;
    Some(Ok(inner(raw, &mut state.output)))
}

fn inner(raw: &str, output: &mut String) -> Value {
    let (numer, denom) = match raw.split_once('.') {
        None => (natural(raw, output), BigUint::one()),
        Some((whole, frac)) => decimal(whole, frac, output),
    };
    Value::from(BigRational::new_raw(numer.into(), denom.into()))
}

fn natural(raw: &str, output: &mut String) -> BigUint {
    let mut rest = raw.trim_start_matches('0');
    if rest.is_empty() {
        rest = "0"
    }
    output.push_str(rest);
    parse_validated::<BigUint>(rest)
}

fn decimal(mut whole: &str, mut frac: &str, output: &mut String) -> (BigUint, BigUint) {
    frac = frac.trim_end_matches('0');
    if frac.is_empty() {
        return (natural(whole, output), BigUint::one());
    }
    whole = whole.trim_start_matches('0');
    if whole.is_empty() {
        whole = "0"
    }
    output.push_str(whole);
    output.push('.');
    output.push_str(frac);
    let offset = u32::try_from(frac.len()).expect("offset should fit into a u32");
    // the input must be truly huge for the offset to not fit into a u32
    let denom = BigUint::from(10u32).pow(offset);
    let whole = parse_validated::<BigUint>(whole);
    let frac = parse_validated::<BigUint>(frac);
    let numer = whole * &denom + frac;
    (numer, denom)
}
