use ark_ff::fields::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "3221225473"]
#[generator = "5"]
pub struct FqConfig;
pub type F = Fp64<MontBackend<FqConfig, 1>>;
