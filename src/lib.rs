//! # complex_algebra
//!
//! This crate intends to support complex numbers and its standard algebraic operations.
//! To construct a complex number with real part `u` and imaginary part `v` you can do
//! ```
//! use complex_algebra::c;
//! let u = 2.0;
//! let v = 3.0;
//! let z = c(u, v);
//! ```
//!
//! `u`, `v` can be any types like `i32`, `u32`, `f64`, ... that implement at the very minimum
//! the traits `Copy` and `PartialEq`.
//!
//! Depending on the chosen type and its support for various algebraic operators,
//! the following binary and unary functions are implemented:
//!
//! z1 + z2
//!
//! z1 - z2
//!
//! z1 * z2
//!
//! z1 / z2
//!
//! -z
//!
//! Moreover, all these binary operations do work when the r.h.s is being replaced with
//! a 'real' number.
//!
//! ## Example:
//! ```
//! use complex_algebra::c;
//! let z1 = c(2, 3);
//! let z2 = c(1, 1);
//!
//! assert_eq!(&z1 + &z2, c(3, 4));
//! assert_eq!(z1 * 2, c(4, 6));
//!
//! ```

mod complex;
pub use complex::*;
use std::ops::Sub;

/// Takes a real and transforms it to a number of type `c`.
/// Corresponds to the embedding of a real number into a complex.
///
/// ```
/// use complex_algebra::{c, re};
/// assert_eq!(re(3), c(3, 0))
/// ```
pub fn re<T: Copy + PartialEq + Sub<Output = T>>(r: T) -> c<T> {
    c(r, r - r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exp() {
        let z = c(0.0, 2.0 * std::f64::consts::PI);
        println!("{:?}", c::exp(z));
    }

    #[test]
    fn test_pow() {
        let z = c(0.0, 1.0);
        println!("{:?}", z.pow(2));
        println!("{:?}", z.pow(3));
    }
}
