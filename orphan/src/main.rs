// This will not compile because it breaks the orphan rule.
// Either the trait or the type must be local to the crate.
//impl std::fmt::Debug for bool {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", if *self { "true" } else { "false" })
//    }
//}

fn main() {
    println!("See the starter package for a solution to the orphan rule");
}
