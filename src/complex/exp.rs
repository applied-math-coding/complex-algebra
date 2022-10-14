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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exp() {
        let z = c(0.0, 2.0 * std::f64::consts::PI);
        println!("{:?}", c::exp(z));
    }
}
