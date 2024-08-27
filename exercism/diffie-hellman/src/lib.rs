use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let mod_exp = |(r, b), e| (if (a >> e) % 2 != 0 { r * b % p } else { r }, b * b % p);
    (0..(64 - a.leading_zeros())).fold((1, g), mod_exp).0
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key(p, b_pub, a)
}
