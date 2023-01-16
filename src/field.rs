use ark_ff::fields::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "3221225473"]
#[generator = "5"]
pub struct FqConfig;
pub type F = Fp64<MontBackend<FqConfig, 1>>;

// `Generator` implementation for `F` type
impl Generator for F {
    /// Returns the generator of the field.
    /// Hard-coded to 5 for the tutorial.
    fn generator() -> Self {
        Self::from(5)
    }
}

pub trait Generator {
    /// Returns the generator of the field.
    fn generator() -> Self;
}
