#![warn(missing_docs)]
//! A simple project demonstrating rustdoc with helper functions.

//! # Examples
//! ----
//! use rustdoc_example::mult;
//! assert_eq!(mult(10, 10), 100);
//!-----


/// Returns the sum of `left` and `right`.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Returns the product of `a` and `b`.
pub fn mult(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        assert_eq! (2 * 2, mult(2, 2));
    }
}
