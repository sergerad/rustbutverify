use itertools::Itertools;

pub fn parentheses(n: usize) -> Vec<Vec<String>> {
    ["(".to_string(), ")".to_string()]
        .into_iter()
        .combinations_with_replacement(n)
        .filter(|ps| {
            ps.iter()
                .fold(0, |acc, p| if p == "(" { acc + 1 } else { acc - 1 })
                == 0
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn parentheses_combinations() {
        let n = 2;
        let ps = parentheses(n);
        assert_eq!(ps, vec![vec!["(".to_string(), ")".to_string()]]);
    }
}
