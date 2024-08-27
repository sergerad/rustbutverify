pub fn nth(n: usize) -> u32 {
    (2..).filter(|&i| is_prime(i)).nth(n).unwrap()
}

pub fn is_prime(p: u32) -> bool {
    !(2..).take_while(|i| i * i <= p).any(|i| p % i == 0)
}
