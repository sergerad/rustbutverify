#[derive(thiserror::Error, Debug)]
pub enum Error {
    // Generic error that we can replace enetually
    #[error("Generic {0}")]
    Generic(String),

    // Errors that map from module errors
    // ...

    // Errors that map from external crates
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
