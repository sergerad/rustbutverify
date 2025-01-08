use itertools::Itertools;

pub fn closest_pair_sum(values: (&[i32], &[i32]), target: i32) -> i32 {
    values
        .0
        .iter()
        .chain(values.1.iter())
        .tuple_combinations()
        .fold(None, |closest_diff, (a, b)| match closest_diff {
            None => Some(a + b),
            Some(diff) => {
                let sum = a + b;
                if (sum - target).abs() < (diff - target).abs() {
                    Some(sum)
                } else {
                    Some(diff)
                }
            }
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::sum_pairs::*;
    #[test]
    fn closest() {
        let values = ([1, 2, 3].as_slice(), [4, 5, 6].as_slice());
        let target = 12;
        let result = closest_pair_sum(values, target);
        assert_eq!(result, 11);
    }
}
