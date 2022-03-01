use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone)]
pub struct c<T: Copy + PartialEq>(T, T);

macro_rules! complex_add {
  ($LHS:ty, $RHS:ty, $T:tt ) => {
    impl<$T: Add<Output = $T> + Copy + PartialEq> Add<$RHS> for $LHS {
      type Output = c<$T>;
      fn add(self, rhs: $RHS) -> Self::Output {
        c(self.0 + rhs.0, self.1 + rhs.1)
      }
    }
  };
}
complex_add!(c<T>, c<T>, T);
complex_add!(&c<T>, &c<T>, T);
complex_add!(c<T>, &c<T>, T);
complex_add!(&c<T>, c<T>, T);

macro_rules! complex_mul {
  ($LHS:ty, $RHS:ty, $T:tt ) => {
    impl<$T: Add<Output = $T> + Mul<Output = $T> + Sub<Output = $T> + Copy + PartialEq> Mul<$RHS>
      for $LHS
    {
      type Output = c<$T>;
      fn mul(self, rhs: $RHS) -> Self::Output {
        c(
          self.0 * rhs.0 - self.1 * rhs.1,
          self.0 * rhs.1 + self.1 * rhs.0,
        )
      }
    }
  };
}
complex_mul!(c<T>, c<T>, T);
complex_mul!(&c<T>, &c<T>, T);
complex_mul!(c<T>, &c<T>, T);
complex_mul!(&c<T>, c<T>, T);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_1() {
    let z1 = c(1f64, 2f64);
    let z2 = c(3f64, 4f64);
    assert_eq!(z1 + z2, c(4.0, 6.0));
  }

  #[test]
  fn test_add_2() {
    let z1 = c(1f64, 2f64);
    let z2 = c(3f64, 4f64);
    let z3 = c(0f64, 0f64);
    assert_eq!(&z1 + &z2 + &z3, c(4.0, 6.0));
  }

  #[test]
  fn test_add_3() {
    let z1 = c(1f64, 2f64);
    let z2 = c(3f64, 4f64);
    let z3 = c(0f64, 0f64);
    assert_eq!(&z1 + (&z2 + &z3), c(4.0, 6.0));
  }

  #[test]
  fn test_mul_1() {
    let z1 = c(3i32, 2i32);
    let z2 = c(1i32, 4i32);
    assert_eq!(z1 * z2, c(-5i32, 14i32));
  }

  #[test]
  fn test_mul_2() {
    let z1 = c(3i32, 2i32);
    let z2 = c(1i32, 4i32);
    assert_eq!(&z1 * &z2, c(-5i32, 14i32));
  }

  #[test]
  fn test_mul_3() {
    let z1 = c(3i32, 2i32);
    let z2 = c(1i32, 4i32);
    assert_eq!(&z1 * z2, c(-5i32, 14i32));
  }

  #[test]
  fn test_mul_4() {
    let z1 = c(3i32, 2i32);
    let z2 = c(1i32, 4i32);
    assert_eq!(z1 * &z2, c(-5i32, 14i32));
  }
}