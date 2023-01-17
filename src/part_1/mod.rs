pub mod fib_square;
pub mod thinking_polynomial;
use crate::field::F;
use eyre::Result;

pub fn run() -> Result<()> {
    println!("Part 1: Trace and Low-Degree Extension");
    run_basics()?;

    run_fibonacci_sq_trace()?;

    run_thinking_of_polynomials()?;

    Ok(())
}

fn run_basics() -> Result<()> {
    println!("The Basics");
    println!(
        "3221225472 + 10 = {}",
        F::from(3221225472_u64) + F::from(10)
    );
    Ok(())
}

fn run_fibonacci_sq_trace() -> Result<()> {
    println!("FibonacciSq Trace");
    let mut fib = fib_square::FibonacciSquare::new(F::from(1), F::from(3141592));
    println!("You should implement the body of compute() in src/part_1/fib_square.rs");
    println!(
        "In case you're stuck or if you want to skip, you can find the solution in src/solutions/part_1/fib_square.rs"
    );
    println!("fib(1023) = {}", fib.compute(1023));
    match fib.compute(1023) == F::from(2338775057_u64) {
        true => {
            println!("Congratulations! You've completed FibonacciSq Trace!");
            Ok(())
        }
        false => Err(eyre::eyre!(
            "Oops! You didn't quite get it right. Try again!"
        )),
    }
}

fn run_thinking_of_polynomials() -> Result<()> {
    println!("Thinking of Polynomials");
    run_find_group()?;

    Ok(())
}

fn run_find_group() -> Result<()> {
    println!("Find a Group of Size 1024");
    println!("You should implement the body of find_group_size_1024() in src/part_1/thinking_polynomial.rs");
    println!(
        "In case you're stuck or if you want to skip, you can find the solution in src/solutions/part_1/thinking_polynomial.rs"
    );
    println!("Hint: When  𝑘  divides  |𝔽×| ,  𝑔𝑘  generates a group of size  |𝔽×|𝑘 , and the n-th power of some FieldElement  𝑥  can be computed by calling x ** n .");
    let (g, G) = thinking_polynomial::find_group_size_1024();
    thinking_polynomial::check_find_group_size_1024(g, G)
}
