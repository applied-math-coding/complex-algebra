use super::c;

pub trait Cos<T: Copy + PartialEq> {
    fn cos(z: c<T>) -> c<T>;
}

#[macro_export]
macro_rules! cos_macro {
    ($T:tt) => {
        impl Cos<$T> for c<$T> {
            fn cos(z: c<$T>) -> c<$T> {
                c(0.5, 0.0) * (c::exp(c(0.0, 1.0) * z) + c::exp(c(0.0, -1.0) * z))
            }
        }
    };
}
