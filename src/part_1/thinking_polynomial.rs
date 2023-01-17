use crate::field::{Generator, F};
use eyre::Result;
/// Find a Group of Size 1024
pub fn find_group_size_1024() -> Vec<F> {
    // Change the following line so that g will generate a group of size 1024
    let g = F::generator();
    // Fill G with the elements of G such that G[i] := g ** i
    let G = vec![];
    G
}

/// Check that G is a group of size 1024
pub fn check_find_group_size_1024(G: Vec<F>) -> Result<()> {
    Ok(())
}
