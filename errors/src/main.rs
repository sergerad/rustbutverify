use std::error::Error;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
struct ExampleError;

impl std::fmt::Display for ExampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "An example error occurred")
    }
}

fn report(error: &(dyn Error + 'static)) {
    eprintln!("Error: {}", error);

    let errors = std::iter::successors(Some(error), |&e| e.source()); // e must be a reference in the closure!

    for (index, error) in errors.enumerate() {
        eprintln!("Cause #{}: {}", index, error);
    }
}

fn main() {
    report(&ExampleError);
}
