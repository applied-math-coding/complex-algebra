mod add;
mod mul;
mod neg;
mod sub;
use std::ops::{Add, Mul, Neg};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub struct c<T: Copy + PartialEq>(T, T);

impl<T: Copy + PartialEq> c<T> {
  #[allow(non_snake_case)]
  pub fn Re(&self) -> T {
    self.0
  }

  #[allow(non_snake_case)]
  pub fn Im(&self) -> T {
    self.1
  }
}

impl<T: Copy + PartialEq + Neg<Output = T>> c<T> {
  pub fn conj(&self) -> c<T> {
    c(self.0, -self.1)
  }
}

impl<T: Copy + PartialEq + Add<Output = T> + Mul<Output = T>> c<T> {
  pub fn r_square(&self) -> T {
    self.0 * self.0 + self.1 * self.1
  }
}

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
