mod add;
mod cos;
mod cosh;
mod div;
mod exp;
mod mul;
mod neg;
mod pow;
mod sin;
mod sinh;
mod sub;
pub use cos::Cos;
pub use cosh::Cosh;
pub use exp::Exp;
pub use pow::Pow;
pub use sin::Sin;
pub use sinh::Sinh;
use std::ops::{Add, Mul, Neg};

use crate::cos_macro;
use crate::cosh_macro;
use crate::exp_macro;
use crate::pow_macro;
use crate::pow_non_neg_macro;
use crate::sin_macro;
use crate::sinh_macro;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct c<T: Copy + PartialEq>(pub T, pub T);

impl<T: Copy + PartialEq> c<T> {
    /// Extracts the real part of the complex number.
    #[allow(non_snake_case)]
    pub fn Re(&self) -> T {
        self.0
    }

    /// Extracts the imaginary part of the complex number.
    #[allow(non_snake_case)]
    pub fn Im(&self) -> T {
        self.1
    }
}

/// Returns the conjugated complex number.
impl<T: Copy + PartialEq + Neg<Output = T>> c<T> {
    pub fn conj(&self) -> c<T> {
        c(self.0, -self.1)
    }
}

/// Returns the square of the euclidean norm: `|z|^2`
impl<T: Copy + PartialEq + Add<Output = T> + Mul<Output = T>> c<T> {
    pub fn r_square(&self) -> T {
        self.0 * self.0 + self.1 * self.1
    }
}

exp_macro!(f64);
exp_macro!(f32);
pow_non_neg_macro!(u8, u32, u64, u128);
pow_macro!(i8, i32, i64, i128; u8, u32, u64, u128; f32);
pow_macro!(i8, i32, i64, i128; u8, u32, u64, u128; f64);
cos_macro!(f32);
cos_macro!(f64);
sin_macro!(f32);
sin_macro!(f64);
sinh_macro!(f32);
sinh_macro!(f64);
cosh_macro!(f32);
cosh_macro!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conj() {
        let z = c(1, 2);
        assert_eq!(z.conj(), c(1, -2));
    }

    #[test]
    fn test_r_square() {
        let z = c(1, 2);
        assert_eq!(z.r_square(), (c(1, 2) * c(1, 2).conj()).Re());
    }
}
