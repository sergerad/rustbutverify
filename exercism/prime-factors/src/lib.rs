pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();

    if let Some(factor) = (2..n + 1).find(|x| n % x == 0) {
        prime_factors.push(factor);
        prime_factors.append(&mut factors(n / factor));
    }
    prime_factors
}
