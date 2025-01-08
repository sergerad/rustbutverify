pub fn k_sized_elem_index(elems: &[u32], k: usize) -> usize {
    let mut v = elems.to_vec();
    v.sort();
    v.into_iter().nth(k - 1).unwrap() as usize
}

#[cfg(test)]
mod tests {
    use crate::kth::*;
    #[test]
    fn kth() {
        let elems = vec![3, 2, 1, 5, 4];
        let k = 2;
        let k_actual = k_sized_elem_index(&elems, k);
        assert_eq!(k_actual, 2);
    }
}
