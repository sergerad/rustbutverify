pub fn raindrops(n: u32) -> String {
    let s = [3, 5, 7].iter().fold(String::new(), |mut acc, &factor| {
        if n % factor == 0 {
            match factor {
                3 => acc.push_str("Pling"),
                5 => acc.push_str("Plang"),
                7 => acc.push_str("Plong"),
                _ => (),
            }
        }
        acc
    });
    if s.is_empty() {
        n.to_string()
    } else {
        s
    }
}
