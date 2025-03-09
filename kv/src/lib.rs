#![doc = include_str!("../README.md")]

mod errors;
mod kv;

pub use errors::Error;
pub use kv::{DurableKv, Key, Ref, RefMut, Value};
