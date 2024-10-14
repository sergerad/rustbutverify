fn main() {
    println!("Hello, world!");
}

// Supertrait.
trait Supertrait: Send + Sync + Copy {}
impl Supertrait for u32 {}

// Must implement supertrait to implement subtrait.
trait Power<E>: Supertrait {
    // The associated type is generic in the trait declaration.
    type Output: Copy;
    fn power(self, exponent: E) -> Self::Output;
}

impl<E> Power<E> for u32
where
    E: Into<u32>,
{
    // The associated type is made concrete within the trait implementation.
    type Output = u32;
    fn power(self, exponent: E) -> Self::Output {
        self.pow(exponent.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }
}
