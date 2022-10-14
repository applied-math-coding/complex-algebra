use super::c;
pub trait Pow<I, T: Copy + PartialEq> {
    fn pow(&self, n: I) -> c<T>;
}

// for n large: z^n = r^n*exp(n*i*phi) with z = r*(cos(phi) + i*sin(phi))
// for n small, iteration of z *= z seems faster
#[macro_export]
macro_rules! pow_macro_single_non_neg {
    ($I:ty, $T:tt) => {
        impl Pow<$I, $T> for c<$T> {
            fn pow(&self, n: $I) -> c<$T> {
                if n == 0 {
                    c(1 as $T, 0 as $T)
                } else if n > 100 {
                    let r = $T::sqrt(self.r_square());
                    let phi = if r.is_normal() {
                        $T::asin(self.Im() / r)
                    } else {
                        0.0
                    };
                    c(r.powf(n as $T), 0.0) * c::exp(c(0.0, phi * n as $T))
                } else {
                    let mut z = *self;
                    for _ in 1..n {
                        z = z * *self;
                    }
                    z
                }
            }
        }
    };
}

#[macro_export]
macro_rules! pow_macro {
    ($($I:ty),*; $($U:ty),*; $T:tt) => {
      $(
        impl Pow<$I, $T> for c<$T> {
          fn pow(&self, n: $I) -> c<$T> {
            let z = self.pow((if n < 0 {-n} else {n}) as $U);
            if n < 0 {c(1.0, 0.0)/z} else {z}
          }
        }
      )*
    }
}

#[macro_export]
macro_rules! pow_non_neg_macro {
($($I:ty),*) => {
  $(
    crate::pow_macro_single_non_neg!($I, f64);
    crate::pow_macro_single_non_neg!($I, f32);
  )*
};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow() {
        let z = c(0.0, 1.0);
        println!("{:?} {:?}", z * z, z.pow(2u8));
        println!("{:?}", z.pow(3));
        println!("{:?} {:?}", z * z * z, z.pow(3));
        let w = c(10.00001, 10.00001);
        println!("{:?} {:?}", w * w * w, w.pow(3));
    }
}
