use super::c;
pub trait Pow<I, T: Copy + PartialEq> {
    fn pow(&self, n: I) -> c<T>;
}

//z^n = r^n*exp(n*i*phi) with z = r*(cos(phi) + i*sin(phi))
#[macro_export]
macro_rules! pow_macro_single {
    ($I:ty, $T:tt) => {
        impl Pow<$I, $T> for c<$T> {
            fn pow(&self, n: $I) -> c<$T> {
                let r = $T::sqrt(self.r_square());
                let phi = if r.is_normal() {
                    $T::asin(self.Im() / r)
                } else {
                    0.0
                };
                c(r.powf(n as $T), 0.0) * c::exp(c(0.0, phi * n as $T))
            }
        }
    };
}

#[macro_export]
macro_rules! pow_macro {
($($I:ty),*) => {
  $(
    crate::pow_macro_single!($I, f64);
    crate::pow_macro_single!($I, f32);
  )*
};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow() {
        let z = c(0.0, -1.0);
        println!("{:?}", z.pow(2));
        println!("{:?}", z.pow(3));
        println!("{:?} {:?}", z * z * z, z.pow(3));
        let w = c(10.00001, 10.00001);
        println!("{:?} {:?}", w * w * w, w.pow(3));
    }
}
