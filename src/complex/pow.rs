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
                    $T::acos(self.Re() / r)
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
