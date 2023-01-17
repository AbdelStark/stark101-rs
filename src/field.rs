use ark_ff::fields::{Fp64, MontBackend, MontConfig};
use ark_std::One;

#[derive(MontConfig)]
#[modulus = "3221225473"]
#[generator = "5"]
pub struct FqConfig;
pub type F = Fp64<MontBackend<FqConfig, 1>>;

// `Generator` implementation for `F` type
impl Generator for F {
    /// Return the generator of the field.
    /// Hard-coded to 5 for the tutorial.
    fn generator() -> Self {
        Self::from(5)
    }
}

// `Order` implementation for `F` type
impl Order for F {
    /// Return if the field is of order n.
    /// Naively checks that the element is of order n by raising it to all powers up to n, checking
    /// that the element to the n-th power is the unit, but not so for any k<n.
    /// # Arguments
    /// * `n` - The order to check.
    /// # Returns
    /// `true` if the field is of order `n`, `false` otherwise.
    fn is_order(&self, n: F) -> bool {
        let one = F::one();
        assert!(n > one);
        let mut i = one;
        let mut h = one;
        while i < n {
            // Multiply h by self
            h = h * self;
            if h == one {
                return false;
            }
            // Increment i
            i = i + one;
        }
        h * self == one
    }
}

pub trait Generator {
    /// Return the generator of the field.
    fn generator() -> Self;
}

pub trait Order {
    /// Return if the field is of order n.
    /// # Arguments
    /// * `n` - The order to check.
    /// # Returns
    /// `true` if the field is of order `n`, `false` otherwise.
    fn is_order(&self, n: F) -> bool;
}
