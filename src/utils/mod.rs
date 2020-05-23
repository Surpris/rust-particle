//! utils
//!
//! utility for the core module

use num_traits::float::Float;
use num_traits::{Num, NumCast};

/// cast a numeric value with type T to one with U
pub fn cast_t2u<T, U>(x: T) -> U
where
    T: Num + NumCast,
    U: Num + NumCast,
{
    U::from(x).unwrap()
}

/// return a scale of the frequency domain
pub fn calc_freq<T>(x: &[T]) -> Vec<T>
where
    T: Float,
{
    assert!(x.len() > 0);
    let df: T = cast_t2u::<f64, T>(1.0) / (x[x.len() - 1] - x[0]);
    (0..x.len()).map(|i| cast_t2u::<usize, T>(i) * df).collect()
}

/// return an all-zero vector with the length `n`
pub fn zeros<T>(n: usize) -> Vec<T> 
where T: Num + NumCast + Copy
{
    vec![cast_t2u::<f64, T>(0.0); n]
}

/// return an all-zero vector with the length same as that of the input
pub fn zeros_like<T>(x: &[T]) -> Vec<T> 
where T: Num + NumCast + Copy
{
    zeros(x.len())
}

/// return an all-one vector with the length `n`
pub fn ones<T>(n: usize) -> Vec<T> 
where T: Num + NumCast + Copy
{
    vec![cast_t2u::<f64, T>(1.0); n]
}

/// return an all-one vector with the length same as that of the input
pub fn ones_like<T>(x: &[T]) -> Vec<T> 
where T: Num + NumCast + Copy
{
    ones(x.len())
}

/// return a [`a`, `b`) vector with a step of  `(b-a)/size`.
/// This function is similar to xrange(a, b, size).
pub fn xrange<T>(a: T, b: T, size: usize) -> Vec<T>
where
    T: Num + NumCast + Copy,
{
    let factor: T = (b - a) / cast_t2u::<usize, T>(size);
    (0..size)
        .map(|i| a + cast_t2u::<usize, T>(i) * factor)
        .collect()
}

/// calculate a [`a`, `b`) vector with a step of `step`.
/// This function is similar to numpy.arange(a, b, step).
pub fn arange<T>(a: T, b: T, step: T) -> Vec<T>
where
    T: Num + NumCast + Copy,
{
    let size = cast_t2u::<f64, usize>(cast_t2u::<T, f64>((b - a) / step).floor());
    (0..size)
        .map(|i| a + cast_t2u::<usize, T>(i) * step)
        .collect()
}

/// calculate a [`a`, `b`) vector with a length of `size`.
/// This function is similar to numpy.linspace(a, b, size).
pub fn linspace<T>(a: T, b: T, size: usize) -> Vec<T>
where
    T: Num + NumCast + Copy,
{
    let factor: T = (b - a) / cast_t2u::<usize, T>(size - 1);
    (0..size)
        .map(|i| a + cast_t2u::<usize, T>(i) * factor)
        .collect()
}
