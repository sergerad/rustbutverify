use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::One;
use std::ops::{Add, AddAssign, Deref, Mul, Sub};

// Represents a field element in the prime field.
#[derive(Clone, Debug)]
pub struct FieldElement {
    value: BigInt,
    prime: BigInt,
}

impl FieldElement {
    // Creates a new prime field element.
    pub fn new(value: BigInt, prime: BigInt) -> Self {
        Self {
            value: value.mod_floor(&prime),
            prime,
        }
    }

    // Computes the multiplicative inverse of the field element.
    pub fn inverse(&self) -> Self {
        let e = self.value.extended_gcd(&self.prime);
        assert!(e.gcd.is_one(), "Value and prime must be coprime.");
        FieldElement::new(e.x.mod_floor(&self.prime), self.prime.clone())
    }

    pub fn prime(&self) -> &BigInt {
        &self.prime
    }
}

impl Deref for FieldElement {
    type Target = BigInt;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.prime, other.prime);
        FieldElement::new(self.value + other.value, self.prime)
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(self.prime, other.prime);
        FieldElement::new(self.value - other.value, self.prime)
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        assert_eq!(self.prime, other.prime);
        FieldElement::new(self.value * other.value, self.prime)
    }
}

impl AddAssign for FieldElement {
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.prime, other.prime);
        self.value = (self.value.clone() + other.value).mod_floor(&self.prime);
    }
}
