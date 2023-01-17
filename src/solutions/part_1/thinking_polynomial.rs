use crate::field::{Generator, F};
use ark_ff::Field;

/// Find a Group of Size 1024
pub fn find_group_size_1024() -> (F, Vec<F>) {
    let m = (3_u64 * 2_u64).pow(20);
    let g = F::generator().pow([m]);
    // Fill G with the elements of G such that G[i] := g ** i
    let mut G = vec![];
    for i in 0..1024 {
        G.push(g.pow([i]));
    }
    (g, G)
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    use super::*;

    #[test]
    fn test_find_group_size_1024() {
        let (g, G) = find_group_size_1024();
        let result = part_1::thinking_polynomial::check_find_group_size_1024(g, G);
        assert!(result.is_ok());
    }
}
