use super::c;

pub trait Exp<T: Copy + PartialEq> {
    fn exp(z: c<T>) -> c<T>;
}

#[macro_export]
macro_rules! exp_macro {
    ($T:tt) => {
        impl Exp<$T> for c<$T> {
            fn exp(z: c<$T>) -> c<$T> {
                c($T::exp(z.Re()), 0.0) * c($T::cos(z.Im()), $T::sin(z.Im()))
            }
        }
    };
}
