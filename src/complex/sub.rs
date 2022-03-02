use super::c;
use std::ops::Sub;

macro_rules! complex_sub {
  ($LHS:ty, $RHS:ty, $T:tt ) => {
    impl<$T: Sub<Output = $T> + Copy + PartialEq> Sub<$RHS> for $LHS {
      type Output = c<$T>;
      fn sub(self, rhs: $RHS) -> Self::Output {
        c(self.0 - rhs.0, self.1 - rhs.1)
      }
    }
  };
}
complex_sub!(c<T>, c<T>, T);
complex_sub!(&c<T>, &c<T>, T);
complex_sub!(c<T>, &c<T>, T);
complex_sub!(&c<T>, c<T>, T);

macro_rules! complex_real_sub {
  ($LHS:ty,  $T:tt ) => {
    impl<$T: Sub<Output = $T> + Copy + PartialEq> Sub<$T> for $LHS {
      type Output = c<$T>;
      fn sub(self, rhs: $T) -> Self::Output {
        c(self.0 - rhs, self.1)
      }
    }
  };
}
complex_real_sub!(c<T>, T);
complex_real_sub!(&c<T>, T);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sub_1() {
    let z1 = c(1f64, 2f64);
    let z2 = c(3f64, 4f64);
    assert_eq!(z1 - z2, c(-2.0, -2.0));
  }

  #[test]
  fn test_sub_2() {
    let z1 = c(1f64, 2f64);
    let z2 = c(3f64, 4f64);
    let z3 = c(0f64, 0f64);
    assert_eq!(&z1 - &z2 - &z3, c(-2.0, -2.0));
  }

  #[test]
  fn test_sub_3() {
    let z1 = c(1f64, 2f64);
    let z2 = c(3f64, 4f64);
    let z3 = c(0f64, 0f64);
    assert_eq!(&z1 - (&z2 - &z3), c(-2.0, -2.0));
  }

  #[test]
  fn test_sub_4() {
    let z = c(1f64, 2f64);
    assert_eq!(z - 1.0, c(0.0, 2.0));
  }

  #[test]
  fn test_sub_5() {
    let z = c(1f64, 2f64);
    assert_eq!(&z - 1.0, c(0.0, 2.0));
  }
}
