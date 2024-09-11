use std::{
    iter::{Product, Sum},
    ops::{Add, Mul, Sub},
};

// Represents a field element in the prime field.
#[derive(Clone, Copy)]
pub struct FieldElement<const P: u128> {
    value: u128,
}

impl<const P: u128> std::fmt::Debug for FieldElement<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const P: u128> FieldElement<P> {
    pub fn new(value: u128) -> Self {
        Self { value: value % P }
    }

    pub fn zero() -> Self {
        Self::new(0u128)
    }

    pub fn one() -> Self {
        Self::new(1u128)
    }
}

impl<const P: u128> From<bool> for FieldElement<P> {
    fn from(b: bool) -> Self {
        if b {
            Self::one()
        } else {
            Self::zero()
        }
    }
}

impl<const P: u128> Add for FieldElement<P> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        FieldElement::new(self.value + other.value)
    }
}

impl<const P: u128> Sub for FieldElement<P> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let a = self.value;
        let b = other.value;
        if a >= b {
            FieldElement::new(a - b)
        } else {
            let diff = b - a;
            FieldElement::new(P - (diff % P))
        }
    }
}

impl<const P: u128> Mul for FieldElement<P> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        FieldElement::new(self.value * other.value)
    }
}

impl<const P: u128> Sum for FieldElement<P> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl<const P: u128> Product for FieldElement<P> {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::one(), |acc, x| acc * x)
    }
}
