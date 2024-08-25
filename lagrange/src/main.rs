mod field;
mod lagrange;
mod polynomial;

use field::FieldElement;
use lagrange::lagrange_interpolation;
use num_bigint::BigInt;

fn main() {
    // Input = (0, 1, 2, 0) over Fp, p=67.
    let p = BigInt::from(67);
    let input = vec![
        FieldElement::new(BigInt::from(0), &p),
        FieldElement::new(BigInt::from(1), &p),
        FieldElement::new(BigInt::from(2), &p),
        FieldElement::new(BigInt::from(0), &p),
    ];

    // Compute the Lagrange interpolation polynomial.
    let l = lagrange_interpolation(input, &p);

    // Evaluate the polynomial at all elements of the prime field.
    let field_len = usize::try_from(p.clone()).unwrap();
    for elem in 0..field_len {
        let x = FieldElement::new(BigInt::from(elem), &p);
        let y = l.evaluate(&x);
        println!("P({}) = {}", *x, *y);
    }
}
