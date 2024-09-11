use bitvec::vec::BitVec;

mod field;
mod hypercube;
mod mle;
mod multivariate;

use field::FieldElement;
use hypercube::Hypercube;
use mle::multilinear_extension;
use multivariate::Multivariate;

fn main() {
    // Select a prime number for Fp.
    const P: u128 = 5u128;

    // Select a v, such that f domain is 2^v.
    let v = 2;

    // Construct the hypercube for the domain.
    let hypercube = Hypercube::new(v);

    // Create an arbitrary multivariate function that maps the hypercube to Fp.
    let multivariate = Multivariate::new(hypercube, |tuple: &BitVec| {
        // Evaluations are simply [1,2,3,...,2^v) mod p.
        let sum = tuple
            .iter()
            .by_vals()
            .enumerate()
            .fold(FieldElement::<P>::one(), |acc, (i, bit)| {
                acc + FieldElement::new(2u128.pow(i as u32)) * FieldElement::from(bit)
            });
        sum
    });
    println!(
        "Computed f over the hypercube {{0,1}}^{v}:\n{:?}",
        multivariate
    );

    // Evaluate the MLE at every point in Fp.
    println!("Computed MLE for every point in Fp:");
    for i in 0..P {
        for j in 0..P {
            let point = vec![FieldElement::new(i), FieldElement::new(j)];
            let mle = multilinear_extension(multivariate.clone(), point);
            print!("{:?} ", mle);
        }
        println!();
    }
}
