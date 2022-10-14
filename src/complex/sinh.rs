use super::c;

pub trait Sinh<T: Copy + PartialEq> {
    fn sinh(z: c<T>) -> c<T>;
}

#[macro_export]
macro_rules! sinh_macro {
    ($T:tt) => {
        impl Sinh<$T> for c<$T> {
            fn sinh(z: c<$T>) -> c<$T> {
                c(1.0, 0.0) / c(0.0, 1.0) * c::sin(c(0.0, 1.0) * z)
            }
        }
    };
}
