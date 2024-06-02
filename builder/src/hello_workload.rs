use crate::worker::*;

// A workload that prints a string
#[derive(Clone, Debug)]
pub struct HelloWorkload<'a>(pub &'a str);

impl Workload for HelloWorkload<'_> {
    fn work(&self) {
        println!("{}", self.0);
    }
}

impl<'a> From<&'a str> for HelloWorkload<'a> {
    fn from(s: &'a str) -> Self {
        HelloWorkload(s)
    }
}
