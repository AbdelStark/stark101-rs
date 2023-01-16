use ark_ff::{Field, PrimeField};
use ark_std::UniformRand;
use rand::thread_rng;
use stark101_rs::field::F;

fn main() {
    println!("Welcome to STARK 101!");

    // We can access the prime modulus associated with `F`:
    let modulus = <F as PrimeField>::MODULUS;
    println!("The modulus is: {modulus}");

    // Instantiate random generator
    let mut rng = thread_rng();

    // We can generate a random field element:
    let a = F::rand(&mut rng);
    println!("a: {a}");

    // We can generate a field element from int value:
    let b = F::from(42);
    println!("b: {b}");

    let c = a.square();
    println!("c: {c}");

    let d = a + b;
    println!("d: {d}");
}
