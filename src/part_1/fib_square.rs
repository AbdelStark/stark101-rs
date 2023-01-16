use crate::field::F;
// Required for the implementation of `square()` on `F`.
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
        // Hint: you can use the `square()` method on `F` to compute the square of a field element.
        todo!()
    }
}
