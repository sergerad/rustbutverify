pub fn is_armstrong_number(num: u32) -> bool {
    match num {
        0 => true,
        _ => {
            let digits = num.checked_ilog10().unwrap() + 1;
            let sum = (1..=digits)
                .map(|i| (u64::from(num / 10u32.pow(i - 1) % 10)).pow(digits))
                .sum::<u64>();
            num == sum.try_into().unwrap_or(0u32)
        }
    }
}
