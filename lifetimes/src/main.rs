// From https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

// Specifies that the returned reference will have the same lifetime
// as the shorter of the two input lifetimes.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    // Is set to string2 subsequently.
    let result;
    {
        let string2 = String::from("xyz");
        // Does not compile because lifetime of string2 is too short.
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
}
