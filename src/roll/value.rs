use super::realize::{self, Realize};
use num::{BigUint, Zero as _};

pub fn main(realize: Realize) -> crate::Value {
    crate::Value::from(realize.value())
}

trait Value {
    fn value(self) -> BigUint;
}

impl Value for Realize {
    fn value(self) -> BigUint {
        match self {
            Self::Under(inner) => inner.value(),
            Self::OverDropped(inner) => inner.value(),
            Self::OverFiltered(inner) => inner.value(),
        }
    }
}

impl Value for realize::RealizeUnder {
    fn value(self) -> BigUint {
        let Self {
            lesser,
            middle,
            greater,
            ..
        } = self;

        if lesser.is_some() || greater.is_some() {
            BigUint::from(middle.len())
        } else {
            middle.iter().sum()
        }
    }
}

impl Value for realize::RealizeOverDropped {
    fn value(self) -> BigUint {
        BigUint::zero()
    }
}

impl Value for realize::RealizeOverFiltered {
    fn value(self) -> BigUint {
        BigUint::zero()
    }
}
