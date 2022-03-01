use super::c;
use std::ops::Neg;

macro_rules! complex_neg {
  ($LHS:ty, $T:tt ) => {
    impl<$T: Neg<Output = $T> + Copy + PartialEq> Neg for $LHS {
      type Output = c<$T>;
      fn neg(self) -> Self::Output {
        c(-self.0, -self.1)
      }
    }
  };
}
complex_neg!(c<T>, T);
complex_neg!(&c<T>, T);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_neg_1() {
    let z = c(1f64, 2f64);
    assert_eq!(-z, c(-1.0, -2.0));
  }

  #[test]
  fn test_neg_2() {
    let z = c(1f64, 2f64);
    assert_eq!(--z, c(1.0, 2.0));
  }
}
