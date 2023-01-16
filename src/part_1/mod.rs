pub mod fib_square;
use crate::field::F;

pub fn run() {
    println!("Part 1: Trace and Low-Degree Extension");
    run_basics();

    //run_fibonacci_sq_trace();
}

fn run_basics() {
    println!("The Basics");
    println!(
        "3221225472 + 10 = {}",
        F::from(3221225472_u64) + F::from(10)
    );
}

fn run_fibonacci_sq_trace() {
    println!("FibonacciSq Trace");
    let mut fib = fib_square::FibonacciSquare::new(F::from(1), F::from(3141592));
    println!("You should implement the body of compute() in src/part_1/fib_square.rs");
    println!("fib(1023) = {}", fib.compute(1023));
    assert!(fib.compute(1023) == F::from(2338775057_u64));
}
