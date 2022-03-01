use super::c;
use std::ops::{Add, Mul, Sub};

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
