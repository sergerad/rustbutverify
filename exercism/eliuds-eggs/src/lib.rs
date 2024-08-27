pub fn egg_count(display_value: u32) -> usize {
    format!("{display_value:b}")
        .chars()
        .filter(|&c| c == '1')
        .count()
}
