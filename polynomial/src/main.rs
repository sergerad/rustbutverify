use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
struct FieldElement {
    value: u64,
    modulus: u64,
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.value)
    }
}

impl FieldElement {
    fn new(value: u64, modulus: u64) -> Self {
        Self {
            value: value % modulus,
            modulus,
        }
    }

    fn pow(self, exp: u64) -> Self {
        let mut acc = 1;
        let mut base = self.value % self.modulus;
        let mut exp = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                acc = (acc * base) % self.modulus;
            }
            exp >>= 1;
            base = (base * base) % self.modulus;
        }
        FieldElement {
            value: acc,
            modulus: self.modulus,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.value * rhs.value, self.modulus)
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.value + rhs.value, self.modulus)
    }
}

fn hash(a: &[FieldElement], x: FieldElement) -> FieldElement {
    a.iter()
        .enumerate()
        .fold(FieldElement::new(0, x.modulus), |y, (i, &a)| {
            y + a * x.pow(i as u64)
        })
}

fn main() {
    let n = 128u64;
    let p = n.pow(2);
    let a: Vec<FieldElement> = (1..=n).map(|a| FieldElement::new(a, p)).collect();
    let x = FieldElement::new(511, p);
    let y = hash(&a, x);
    println!("h(a, x={}, p={}) = {}", x, p, y);
}
