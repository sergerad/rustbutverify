#![allow(unused)] // Remove when done

use crate::prelude::*;
use std::fs::{read_dir, ReadDir};

mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    // Print dirs
    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry = NewType(&entry);
        println!("{:?}", *entry);
    }

    println!("{}", Error::Generic("hellowirl".to_string()));

    Ok(())
}
