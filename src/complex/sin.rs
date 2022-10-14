use super::c;

pub trait Sin<T: Copy + PartialEq> {
    fn sin(z: c<T>) -> c<T>;
}

#[macro_export]
macro_rules! sin_macro {
    ($T:tt) => {
        impl Sin<$T> for c<$T> {
            fn sin(z: c<$T>) -> c<$T> {
                c(1.0, 0.0) / c(0.0, 2.0) * (c::exp(c(0.0, 1.0) * z) - c::exp(c(0.0, -1.0) * z))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exp() {
        let z = c(2.0 * std::f64::consts::PI, 0.0);
        println!("{:?}", c::sin(z));
    }
}
