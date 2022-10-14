use super::c;

pub trait Cosh<T: Copy + PartialEq> {
    fn cosh(z: c<T>) -> c<T>;
}

#[macro_export]
macro_rules! cosh_macro {
    ($T:tt) => {
        impl Cosh<$T> for c<$T> {
            fn cosh(z: c<$T>) -> c<$T> {
                c::cos(c(0.0, 1.0) * z)
            }
        }
    };
}
