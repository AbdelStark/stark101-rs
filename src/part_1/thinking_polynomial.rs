use crate::field::{Generator, Order, F};
use ark_std::One;
use eyre::Result;

/// Find a Group of Size 1024
pub fn find_group_size_1024() -> (F, Vec<F>) {
    // Change the following line so that g will generate a group of size 1024
    let g = F::generator();
    // Fill G with the elements of G such that G[i] := g ** i
    let G = vec![];
    (g, G)
}

/// Check that G is a group of size 1024
pub fn check_find_group_size_1024(g: F, G: Vec<F>) -> Result<()> {
    // Check that g is of order 1024
    match g.is_order(F::from(1024)) {
        true => Ok(()),
        false => Err(eyre::eyre!("The generator g is of wrong order.")),
    }?;
    let one = F::one();
    let mut b = one;
    for i in 0..1023 {
        if b != G[i] {
            return Err(eyre::eyre!(
                "The i-th place in G is not equal to the i-th power of g."
            ));
        }
        b *= g;
        if b == one {
            return Err(eyre::eyre!("g is of order {}", i + 1));
        }
    }

    if b * g != one {
        return Err(eyre::eyre!("g is of order > 1024"));
    }

    Ok(())
}
