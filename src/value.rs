use crate::EvalError;
use num::{BigInt, BigRational, BigUint, One as _, Zero as _};
use std::{convert::Infallible, fmt};

#[derive(Debug)]
pub struct Value(BigRational);

impl From<BigRational> for Value {
    fn from(value: BigRational) -> Self {
        Self(value)
    }
}

impl From<BigInt> for Value {
    fn from(value: BigInt) -> Self {
        Self::from(BigRational::new_raw(value, BigInt::one()))
    }
}

impl From<BigUint> for Value {
    fn from(value: BigUint) -> Self {
        Self::from(BigInt::from(value))
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        if b {
            Value(BigRational::one())
        } else {
            Value(BigRational::zero())
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use num::ToPrimitive as _;

        let Self(inner) = self;

        let approx = inner
            .to_f64()
            .expect("value should be representable as an `f64`");
        write!(f, "{approx}",)
    }
}

impl Value {
    fn into_inner(self) -> BigRational {
        let Self(inner) = self;
        inner
    }

    fn as_inner(&self) -> &BigRational {
        let Self(inner) = self;
        inner
    }

    fn is_zero(&self) -> bool {
        self.as_inner().is_zero()
    }

    pub fn eq(self, rhs: Self) -> Result<Self, Infallible> {
        Ok(Self::from(self.into_inner() == rhs.into_inner()))
    }

    pub fn ne(self, rhs: Self) -> Result<Self, Infallible> {
        Ok(Self::from(self.into_inner() != rhs.into_inner()))
    }

    pub fn lt(self, rhs: Self) -> Result<Self, Infallible> {
        Ok(Self::from(self.into_inner() < rhs.into_inner()))
    }

    pub fn le(self, rhs: Self) -> Result<Self, Infallible> {
        Ok(Self::from(self.into_inner() <= rhs.into_inner()))
    }

    pub fn gt(self, rhs: Self) -> Result<Self, Infallible> {
        Ok(Self::from(self.into_inner() > rhs.into_inner()))
    }

    pub fn ge(self, rhs: Self) -> Result<Self, Infallible> {
        Ok(Self::from(self.into_inner() >= rhs.into_inner()))
    }

    pub fn add(self, rhs: Self) -> Result<Self, Infallible> {
        Ok(Self::from(self.into_inner() + rhs.into_inner()))
    }

    pub fn sub(self, rhs: Self) -> Result<Self, Infallible> {
        Ok(Self::from(self.into_inner() - rhs.into_inner()))
    }

    pub fn mul(self, rhs: Self) -> Result<Self, Infallible> {
        Ok(Self::from(self.into_inner() * rhs.into_inner()))
    }

    pub fn div(self, rhs: Self) -> Result<Self, DivisionByZeroError> {
        if rhs.is_zero() {
            return Err(DivisionByZeroError);
        }
        Ok(Self::from(self.into_inner() / rhs.into_inner()))
    }

    pub fn not(self) -> Result<Self, Infallible> {
        Ok(Self::from(self.is_zero()))
    }

    pub fn neg(self) -> Result<Self, Infallible> {
        Ok(Self::from(-self.into_inner()))
    }
}

#[derive(Debug)]
pub enum ValueError {
    DivisionByZero(DivisionByZeroError),
}

impl From<ValueError> for EvalError {
    fn from(error: ValueError) -> Self {
        Self::Value(error)
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZero(error) => write!(f, "{error}"),
        }
    }
}

impl From<DivisionByZeroError> for ValueError {
    fn from(error: DivisionByZeroError) -> Self {
        Self::DivisionByZero(error)
    }
}

impl From<DivisionByZeroError> for EvalError {
    fn from(error: DivisionByZeroError) -> Self {
        Self::Value(ValueError::from(error))
    }
}

#[derive(Debug)]
pub struct DivisionByZeroError;

impl fmt::Display for DivisionByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "division by zero")
    }
}
