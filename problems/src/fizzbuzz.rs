#[derive(Debug, PartialEq)]
enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
}

pub fn fizz_buzz(n: u32) -> Option<FizzBuzz> {
    let mod_3 = n % 3;
    let mod_5 = n % 5;
    match (mod_3, mod_5) {
        (0, 0) => Some(FizzBuzz::FizzBuzz),
        (0, _) => Some(FizzBuzz::Fizz),
        (_, 0) => Some(FizzBuzz::Buzz),
        (_, _) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz::*;
    #[test]
    fn fb() {
        assert_eq!(fizz_buzz(3), Some(FizzBuzz::Fizz));
        assert_eq!(fizz_buzz(5), Some(FizzBuzz::Buzz));
        assert_eq!(fizz_buzz(15), Some(FizzBuzz::FizzBuzz));
    }
}
