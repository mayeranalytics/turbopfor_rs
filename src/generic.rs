/// Generic functions
use crate::codec::*;
use std::marker::PhantomData;
use rand::{
    Rng,
    prelude::Distribution, 
    distributions::{uniform::SampleUniform, Standard}
};

pub trait Encoding {
    type T;
    const ENC_TYPE: u8; // useful for identifying the encoding type in lookup tables, etc.
    fn encode<EW: Width>(input: &[Self::T], output: &mut [u8]) -> usize;
    fn decode<EW: Width>(input: &[u8], n: usize,  output: &mut [Self::T]) -> usize;
    fn sample(len: usize) -> Vec<Self::T> where Self::T: Distribution<Self::T>;
}

pub struct StamdardEncoding<W, T>             { _marker_w: PhantomData<W>, _marker_t: PhantomData<T> }
pub struct IncreasingEncoding<W, T>         { _marker_w: PhantomData<W>, _marker_t: PhantomData<T> }
pub struct StrictlyIncreasingEncoding<W, T> { _marker_w: PhantomData<W>, _marker_t: PhantomData<T> }
pub struct ZigZagEncoding<W, T>             { _marker_w: PhantomData<W>, _marker_t: PhantomData<T> }

// Implement the Encoding trait for each encoding type, linking to the right Codec methods
impl<W: Width, T: Codec<W>> Encoding for StamdardEncoding<W, T> 
    where Standard: Distribution<T>
{
    type T = T;
    const ENC_TYPE: u8 = 0; // Example encoding type constant
    fn encode<EW: Width>(input: &[Self::T], output: &mut [u8]) -> usize {
        T::enc(input, output)
    }
    fn decode<EW: Width>(input: &[u8], n: usize, output: &mut [Self::T]) -> usize {
        T::dec(input, n, output)
    }
    fn sample(len: usize) -> Vec<Self::T> where Self::T: Distribution<Self::T> {
        (0..len).map(|_| rand::random()).collect()
    }
}

impl<W: Width, T: Codec<W>> Encoding for IncreasingEncoding<W, T> 
where Standard: Distribution<T>, T: Copy + std::ops::Add<Output=T> + SampleUniform + PartialOrd + From<u8>
{
    type T = T;
    const ENC_TYPE: u8 = 1;
    fn encode<EW: Width>(input: &[Self::T], output: &mut [u8]) -> usize {
        T::denc(input, output)
    }
    fn decode<EW: Width>(input: &[u8], n: usize, output: &mut [Self::T]) -> usize {
        T::ddec(input, n, output)
    }
    fn sample(len: usize) -> Vec<Self::T>
        where Self::T: Distribution<Self::T> + Copy,
    {
        let mut rng = rand::thread_rng();
        let mut result = Vec::with_capacity(len);
        // Start with an initial random value
        let mut current: T = rng.gen::<Self::T>();
        result.push(current);
        // Define a reasonable delta range to ensure values increase without overflow
        for _ in 1..len {
            // Generate a small delta to add to the current value
            let delta: T = rng.gen_range(T::from(0u8)..=T::from(10u8)); // Delta range from 1 to 10
            current = current + delta;
            result.push(current);
        }
        result
    }
}

impl<W: Width, T: Codec<W>> Encoding for StrictlyIncreasingEncoding<W, T> 
    where Standard: Distribution<T>, T: Copy + std::ops::Add<Output=T> + SampleUniform + PartialOrd + From<u8>
{
    type T = T;
    const ENC_TYPE: u8 = 2;
    fn encode<EW: Width>(input: &[Self::T], output: &mut [u8]) -> usize {
        T::d1enc(input, output)
    }
    fn decode<EW: Width>(input: &[u8], n: usize, output: &mut [Self::T]) -> usize {
        T::d1dec(input, n, output)
    }
    fn sample(len: usize) -> Vec<Self::T>
        where Self::T: Distribution<Self::T> + Copy,
    {
        let mut rng = rand::thread_rng();
        let mut result = Vec::with_capacity(len);
        // Start with an initial random value
        let mut current: T = rng.gen::<Self::T>();
        result.push(current);
        // Define a reasonable delta range to ensure values increase without overflow
        for _ in 1..len {
            // Generate a small delta to add to the current value
            let delta: T = rng.gen_range(T::from(1u8)..=T::from(10u8)); // Delta range from 1 to 10
            current = current + delta;
            result.push(current);
        }
        result
    }
}

impl<W: Width, T: Codec<W>> Encoding for ZigZagEncoding<W, T> 
    where Standard: Distribution<T>
{
    type T = T;
    const ENC_TYPE: u8 = 3;
    fn encode<EW: Width>(input: &[Self::T], output: &mut [u8]) -> usize {
        T::zenc(input, output)
    }
    fn decode<EW: Width>(input: &[u8], n: usize, output: &mut [Self::T]) -> usize {
        T::zdec(input, n, output)
    }
    fn sample(len: usize) -> Vec<Self::T> where Self::T: Distribution<Self::T> {
        (0..len).map(|_| rand::random()).collect()
    }
}

/// Generic encoding function
pub fn encode<E: Encoding, W: Width>(input: &[E::T], output: &mut [u8]) -> usize {
    E::encode::<W>(input, output)
}

/// Generic decoding function
pub fn decode<E: Encoding, W: Width>(input: &[u8], n: usize, output: &mut [E::T]) -> usize {
    E::decode::<W>(input, n, output)
}

#[cfg(test)]
fn test_enc_dec_generic<T, E, WidthType>()
where
    WidthType: Width,
    E: Encoding<T = T>,
    T: Codec<WidthType>,
    T: Eq + Clone + core::fmt::Debug + Default,
    u8: Codec<WidthType>, u16: Codec<WidthType>, u32: Codec<WidthType>, u64: Codec<WidthType>,
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    for _ in 0..256 {
        // Generate random input data
        let len = rng.gen_range(1..=16 * 1024);
        let input: Vec<T> = (0..len).map(|_| rng.gen()).collect();

        // Prepare output buffer for encoding
        let mut buf = vec![0u8; WidthType::enc_buf_size::<T>(input.len())];

        // Encode the data
        let size_enc = encode::<E, WidthType>(&input, &mut buf);

        // Prepare output buffer for decoding
        let mut output: Vec<T> = vec![T::default(); WidthType::dec_buf_len::<T>(input.len())];

        // Decode the data
        let size_dec = decode::<E, WidthType>(&buf[..size_enc], input.len(), &mut output);

        // Check that the encoded and decoded data sizes match
        assert_eq!(size_enc, size_dec);

        // Check that decoded output matches the original input
        assert_eq!(input, output[..input.len()]);
    }
}

#[test]
fn test_enc_dec_u8_normal() {
    test_enc_dec_generic::<u8, StamdardEncoding<W, u8>, W>()
}

#[test]
fn test_enc_dec_u16_normal() {
    test_enc_dec_generic::<u16, StamdardEncoding<W, u16>, W>()
}

#[test]
fn test_enc_dec_u32_normal() {
    test_enc_dec_generic::<u32, StamdardEncoding<W, u32>, W>()
}

#[test]
fn test_enc_dec_u64_normal() {
    test_enc_dec_generic::<u64, StamdardEncoding<W, u64>, W>()
}

#[test]
fn test_enc_dec_u8_increasing() {
    test_enc_dec_generic::<u8, IncreasingEncoding<W, u8>, W>()
}

#[test]
fn test_enc_dec_u16_increasing() {
    test_enc_dec_generic::<u16, IncreasingEncoding<W, u16>, W>()
}

#[test]
fn test_enc_dec_u32_increasing() {
    test_enc_dec_generic::<u32, IncreasingEncoding<W, u32>, W>()
}

#[test]
fn test_enc_dec_u64_increasing() {
    test_enc_dec_generic::<u64, IncreasingEncoding<W, u64>, W>()
}

#[test]
fn test_enc_dec_u8_strictly_increasing() {
    test_enc_dec_generic::<u8, StrictlyIncreasingEncoding<W, u8>, W>()
}

#[test]
fn test_enc_dec_u16_strictly_increasing() {
    test_enc_dec_generic::<u16, StrictlyIncreasingEncoding<W, u16>, W>()
}

#[test]
fn test_enc_dec_u32_strictly_increasing() {
    test_enc_dec_generic::<u32, StrictlyIncreasingEncoding<W, u32>, W>()
}

#[test]
fn test_enc_dec_u64_strictly_increasing() {
    test_enc_dec_generic::<u64, StrictlyIncreasingEncoding<W, u64>, W>()
}

#[test]
fn test_enc_dec_u8_zigzag() {
    test_enc_dec_generic::<u8, ZigZagEncoding<W, u8>, W>()
}

#[test]
fn test_enc_dec_u16_zigzag() {
    test_enc_dec_generic::<u16, ZigZagEncoding<W, u16>, W>()
}

#[test]
fn test_enc_dec_u32_zigzag() {
    test_enc_dec_generic::<u32, ZigZagEncoding<W, u32>, W>()
}

#[test]
fn test_enc_dec_u64_zigzag() {
    test_enc_dec_generic::<u64, ZigZagEncoding<W, u64>, W>()
}
