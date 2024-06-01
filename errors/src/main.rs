use std::error::Error;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
struct ExampleError;

impl std::fmt::Display for ExampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ExampleError occurred")
    }
}

#[derive(Debug, ThisError)]
enum EnumError {
    Example(#[from] ExampleError),
}

impl std::fmt::Display for EnumError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EnumError::Example(_) => write!(f, "EnumErrror::Example occurred"),
        }
    }
}

fn report(error: &(dyn Error + 'static)) {
    eprintln!("Error: {}", error);

    let errors = std::iter::successors(Some(error), |&e| e.source()); // e must be a reference in the closure!

    for (index, error) in errors.enumerate() {
        eprintln!("Cause #{}: {}", index, error);
    }
}

fn map_me() -> Result<(), ExampleError> {
    Err(ExampleError {})
}

fn main() {
    report(&ExampleError);
    report(&EnumError::Example(ExampleError));

    let m = map_me().map_err(EnumError::Example);
    if let Err(ref e) = m {
        report(e);
    }
}
