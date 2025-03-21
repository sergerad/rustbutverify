mod graph;
mod parse;

use graph::Graph;
use std::{
    io::{self, Read},
    process::ExitCode,
};

fn main() -> anyhow::Result<ExitCode> {
    // Read stdin.
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Parse the graph.
    let graph = match Graph::<String>::try_from(buffer.as_str()) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{e}");
            // Exit code 2 for parsing errors.
            return Ok(2.into());
        }
    };

    // Detect cycles and respond accordingly.
    if let Some(cycle) = graph.find_cycle() {
        cycle.iter().for_each(|node| println!("{node}"));
        Ok(1.into())
    } else {
        Ok(0.into())
    }
}
