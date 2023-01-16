use crate::field::F;
use ark_ff::Field;
/// Fibonacci Square implementation
pub struct FibonacciSquare {
    pub first_element: F,
    pub second_element: F,
    pub sequence: Vec<F>,
}

impl FibonacciSquare {
    /// Create a new FibonacciSquare instance given the first two elements of the sequence.
    /// # Arguments
    /// * `first_element` - The first element of the sequence.
    /// * `second_element` - The second element of the sequence.
    /// # Returns
    /// A new FibonacciSquare instance.
    pub fn new(first_element: F, second_element: F) -> Self {
        Self {
            first_element,
            second_element,
            sequence: vec![first_element, second_element],
        }
    }

    /// Compute the nth element of the sequence.
    /// # Arguments
    /// * `n` - The index of the element to compute.
    /// # Returns
    /// The nth element of the sequence.
    pub fn compute(&mut self, n: usize) -> F {
        match n {
            0 => self.first_element,
            1 => self.second_element,
            _ => {
                let mut n_i_minus_2 = self.first_element;
                let mut n_i_minus_1 = self.second_element;
                for _ in 2..n {
                    let n = n_i_minus_2.square() + n_i_minus_1.square();
                    self.sequence.push(n);
                    n_i_minus_2 = n_i_minus_1;
                    n_i_minus_1 = n;
                }
                n_i_minus_1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_square() {
        let mut fib = FibonacciSquare::new(F::from(1), F::from(3141592));
        assert_eq!(fib.compute(1023), F::from(2338775057_u64));
    }
}
