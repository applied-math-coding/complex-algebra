use super::c;
use std::ops::{Add, Div, Mul, Sub};

macro_rules! complex_div {
  ($LHS:ty, $RHS:ty, $T:tt ) => {
    impl<$T: Add<Output = $T> + Mul<Output = $T> + Sub<Output = $T> + Div<Output=T> + Copy + PartialEq> Div<$RHS>
      for $LHS
    {
      type Output = c<$T>;
      fn div(self, rhs: $RHS) -> Self::Output {
        let r = rhs.0*rhs.0+rhs.1*rhs.1;
        c(
          (self.0 * rhs.0 + self.1 * rhs.1)/r,
          (self.1 * rhs.0 - self.0 * rhs.1)/r,
        )
      }
    }
  };
}
complex_div!(c<T>, c<T>, T);
complex_div!(&c<T>, &c<T>, T);
complex_div!(c<T>, &c<T>, T);
complex_div!(&c<T>, c<T>, T);

macro_rules! complex_real_div {
  ($LHS:ty, $T:tt ) => {
    impl<$T: Add<Output = $T> + Mul<Output = $T> + Sub<Output = $T> + Div<Output=T> + Copy + PartialEq> Div<$T>
      for $LHS
    {
      type Output = c<$T>;
      fn div(self, rhs: $T) -> Self::Output {
        c(self.0/rhs, self.1/rhs)
      }
    }
  };
}
complex_real_div!(c<T>, T);
complex_real_div!(&c<T>, T);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_div_1() {
    let z1 = c(3f64, 4f64);
    let z2 = c(8f64, -2f64);
    assert_eq!(z1 / z2, c(4.0 / 17.0, 19.0 / 34.0));
  }

  #[test]
  fn test_div_2() {
    let z1 = c(3f64, 4f64);
    let z2 = c(8f64, -2f64);
    assert_eq!(z1 / &z2, c(4.0 / 17.0, 19.0 / 34.0));
  }

  #[test]
  fn test_div_3() {
    let z1 = c(3f64, 4f64);
    let z2 = c(8f64, -2f64);
    assert_eq!(&z1 / z2, c(4.0 / 17.0, 19.0 / 34.0));
  }

  #[test]
  fn test_div_4() {
    let z1 = c(3f64, 4f64);
    let z2 = c(8f64, -2f64);
    assert_eq!(&z1 / &z2, c(4.0 / 17.0, 19.0 / 34.0));
  }

  #[test]
  fn test_div_5() {
    let z = c(3f64, 4f64);
    assert_eq!(z / 3.0, c(1.0, 4.0 / 3.0));
  }

  #[test]
  fn test_div_6() {
    let z = c(3f64, 4f64);
    assert_eq!(&z / 3.0, c(1.0, 4.0 / 3.0));
  }
}
