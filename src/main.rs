use ark_ff::PrimeField;
use ark_std::UniformRand;
use rand::thread_rng;
use stark101_rs::{
    field::{Generator, F},
    part_1,
};

fn main() {
    println!("Welcome to STARK 101!");

    // We can access the prime modulus associated with `F`:
    let modulus = <F as PrimeField>::MODULUS;
    println!("The modulus is: {modulus}");
    let generator = F::generator();
    println!("The generator is: {generator}");

    // Instantiate random generator
    let mut rng = thread_rng();

    // We can generate a random field element:
    let _ = F::rand(&mut rng);

    // Run the first part of the STARK101 tutorial
    part_1::run();
}
