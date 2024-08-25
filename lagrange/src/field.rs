use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::One;
use std::ops::{Add, AddAssign, Deref, Mul, Sub};

// Represents a field element in the prime field.
#[derive(Clone)]
pub struct FieldElement<'a> {
    value: BigInt,
    prime: &'a BigInt,
}

impl std::fmt::Debug for FieldElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'a> FieldElement<'a> {
    // Creates a new prime field element.
    pub fn new(value: BigInt, prime: &'a BigInt) -> Self {
        Self {
            value: value.mod_floor(prime),
            prime,
        }
    }

    // Computes the multiplicative inverse of the field element.
    pub fn inverse(&self) -> Self {
        let e = self.value.extended_gcd(self.prime);
        assert!(e.gcd.is_one(), "Value and prime must be coprime.");
        FieldElement::new(e.x.mod_floor(self.prime), self.prime)
    }

    pub fn prime(&self) -> &BigInt {
        self.prime
    }
}

impl<'a> Deref for FieldElement<'a> {
    type Target = BigInt;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> Add for FieldElement<'a> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.prime, other.prime);
        FieldElement::new(self.value + other.value, self.prime)
    }
}

impl<'a> Sub for FieldElement<'a> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(self.prime, other.prime);
        FieldElement::new(self.value - other.value, self.prime)
    }
}

impl<'a> Mul for FieldElement<'a> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        assert_eq!(self.prime, other.prime);
        FieldElement::new(self.value * other.value, self.prime)
    }
}

impl<'a> AddAssign for FieldElement<'a> {
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.prime, other.prime);
        self.value = (self.value.clone() + other.value).mod_floor(self.prime);
    }
}
