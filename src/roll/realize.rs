mod display;

use super::parse::Parse;
use num::{BigUint, Zero};
use rand::Rng;
use std::fmt;

#[derive(Debug)]
pub enum Realize {
    Under(RealizeUnder),
    OverDropped(RealizeOverDropped),
    OverFiltered(RealizeOverFiltered),
}

#[derive(Debug)]
pub struct RealizeUnder {
    pub least: Option<Box<[BigUint]>>,
    pub lesser: Option<Box<[BigUint]>>,
    pub middle: Box<[BigUint]>,
    pub greater: Option<Box<[BigUint]>>,
    pub greatest: Option<Box<[BigUint]>>,
}

#[derive(Debug)]
pub struct RealizeOverDropped {
    pub least: Option<Box<[BigUint]>>,
    pub middle: Box<[BigUint]>,
    pub greatest: Option<Box<[BigUint]>>,
}

#[derive(Debug)]
pub struct RealizeOverFiltered {
    pub least: Option<Box<[BigUint]>>,
    pub lesser: Box<[BigUint]>,
    pub middle: Box<[BigUint]>,
    pub greater: Box<[BigUint]>,
    pub greatest: Option<Box<[BigUint]>>,
}

#[derive(Debug)]
pub enum RealizeError {
    DieSizeIsZero,
    RollLenExceedsMaximum,
}

impl fmt::Display for RealizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DieSizeIsZero => write!(f, "die size is zero"),
            Self::RollLenExceedsMaximum => write!(f, "roll length exceeds maximum"),
        }
    }
}

pub fn main(parse: Parse, rng: &mut impl Rng) -> Result<Realize, RealizeError> {
    use num::One as _;
    use rand::distributions::Distribution as _;

    let Parse {
        roll_len,
        die_size,
        drop_least,
        drop_greatest,
        count_greater,
        count_lesser,
    } = parse;
    if die_size.is_zero() {
        return Err(RealizeError::DieSizeIsZero);
    }
    let roll_len = usize::try_from(roll_len)
        .ok()
        .ok_or(RealizeError::RollLenExceedsMaximum)?;
    // since `roll_len` can only be at most `usize::MAX`, `drop_least` and `drop_greatest` need to only be at most `usize::MAX` too
    let mut drop_least = drop_least.map(saturating_cast);
    let mut drop_greatest = drop_greatest.map(saturating_cast);
    let mut roll = rand::distributions::Uniform::new_inclusive(BigUint::one(), die_size)
        .sample_iter(&mut *rng)
        .take(roll_len)
        .collect::<Vec<_>>();
    roll.sort();
    let over_dropped = drop_least.unwrap_or(0) + drop_greatest.unwrap_or(0) >= roll.len();
    if over_dropped {
        let drop_least_inverted = drop_least.map(|amount| roll_len.saturating_sub(amount));
        let drop_greatest_inverted = drop_greatest.map(|amount| roll_len.saturating_sub(amount));
        drop_least = drop_greatest_inverted;
        drop_greatest = drop_least_inverted;
    }
    let greatest = drop_greatest.map(|amount| finalize(take_greatest(&mut roll, amount), rng));
    let least = drop_least.map(|amount| finalize(take_least(&mut roll, amount), rng));
    if over_dropped {
        return Ok(Realize::OverDropped(RealizeOverDropped {
            least,
            middle: finalize(roll, rng),
            greatest,
        }));
    }
    let overlapping_bounds = count_greater
        .as_ref()
        .zip(count_lesser.as_ref())
        .filter(|(count_greater, count_lesser)| count_greater > count_lesser);
    if let Some((count_greater, count_lesser)) = overlapping_bounds {
        // swap thresholds by swapping where they're used
        let greater = finalize(take_greater(&mut roll, count_greater), rng);
        let lesser = finalize(take_lesser(&mut roll, count_lesser), rng);
        return Ok(Realize::OverFiltered(RealizeOverFiltered {
            least,
            lesser,
            middle: finalize(roll, rng),
            greater,
            greatest,
        }));
    }
    let greater = count_lesser.map(|threshold| finalize(take_greater(&mut roll, &threshold), rng));
    let lesser = count_greater.map(|threshold| finalize(take_lesser(&mut roll, &threshold), rng));
    Ok(Realize::Under(RealizeUnder {
        least,
        lesser,
        middle: finalize(roll, rng),
        greater,
        greatest,
    }))
}

fn finalize(mut roll: Vec<BigUint>, rng: &mut impl Rng) -> Box<[BigUint]> {
    use rand::seq::SliceRandom;

    roll.shuffle(rng);
    roll.into_boxed_slice()
}

// todo this should really be part of the `num` crate
fn saturating_cast(value: BigUint) -> usize {
    usize::try_from(value).unwrap_or(usize::MAX)
}

fn take_greatest<T>(sorted: &mut Vec<T>, amount: usize) -> Vec<T> {
    split_off_back(sorted, sorted.len() - amount)
}

fn take_least<T>(sorted: &mut Vec<T>, amount: usize) -> Vec<T> {
    split_off_front(sorted, amount)
}

fn take_greater<T: Ord>(sorted: &mut Vec<T>, threshold: &T) -> Vec<T> {
    let at = sorted.partition_point(|value| value < threshold);
    split_off_back(sorted, at)
}

fn take_lesser<T: Ord>(sorted: &mut Vec<T>, threshold: &T) -> Vec<T> {
    let at = sorted.partition_point(|value| value <= threshold);
    split_off_front(sorted, at)
}

fn split_off_front<T>(vec: &mut Vec<T>, at: usize) -> Vec<T> {
    let mut split_off = split_off_back(vec, at);
    std::mem::swap(&mut split_off, vec);
    split_off
}

fn split_off_back<T>(vec: &mut Vec<T>, at: usize) -> Vec<T> {
    vec.split_off(at)
}
