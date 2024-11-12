/// Generic functions
use std::ops::{Add, Sub};
use rand::{
    Rng,
    prelude::Distribution, 
    distributions::{uniform::SampleUniform, Standard}
};
use num_traits::Bounded;

/// Helper trait for the integer arithmetic we need
pub trait Arithmetic:
    Add<Output = Self> + Sub<Output = Self> 
    + Sized + Copy + Bounded + PartialOrd + From<u8> + SampleUniform
    {}

/// Implement for all possible types that satisfy the constraints
impl<T> Arithmetic for T where
    T: Add<Output = T> + Sub<Output = T> 
    + Sized + Copy + Bounded + PartialOrd + From<u8> + SampleUniform
    {}

/// Sample random number from `T`.
pub fn sample_standard<T>(len: usize) -> Vec<T>
where T: Arithmetic, Standard: Distribution<T>
{
    (0..len).map(|_| rand::random()).collect()
}

/// Sample increasing and strictly increasing data. Use `delta_min`=0 for increasing and `delta_min`=1 for strictly increasing.
pub fn sample_increasing<T>(len: usize, delta_min: u8, delta_max: u8) -> Vec<T>
where T: Arithmetic, Standard: Distribution<T>
{
    let mut rng = rand::thread_rng();
    let mut result = Vec::with_capacity(len);
    // Start with an initial random value
    let mut current: T = rng.gen::<T>();
    result.push(current);
    // Define a reasonable delta range to ensure values increase without overflow
    for filled_len in 0..(len-1) {
        // Generate a small delta and ensure it doesn't overflow
        let delta: T = rng.gen_range(T::from(delta_min)..=T::from(delta_max));
        if current > T::max_value() - delta {
            result.truncate(filled_len);
            break;
        }
        current = current + delta;
        result.push(current);
    }
    result
}
