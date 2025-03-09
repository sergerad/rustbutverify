/// Enumerates all errors that can be returned
/// by the [`crate::DurableKv`] runtime.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    ///...
    #[error(transparent)]
    Encoding(#[from] bincode::Error),
    ///...
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
