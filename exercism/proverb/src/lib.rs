use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        _ => list
            .windows(2)
            .map(|pair| format!("For want of a {} the {} was lost.\n", pair[0], pair[1]))
            .chain(once(format!("And all for the want of a {}.", list[0])))
            .collect(),
    }
}
