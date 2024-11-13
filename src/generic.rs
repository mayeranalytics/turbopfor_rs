/// Generic functions `encode` and `decode` that taken an instance the `Encoding` trait.
/// The encode and decode come also in the flavour Encoding::encode and Encoding::decode.
/// 
/// The point of the whole commotion is to have single generic encode<Encoding> and decode<Encoding>
/// function that is better suited to generic programming than the wrappers codec::enc, dec, etc.
///
/// There are four such `Encoding` instances:
/// - `StandardEncoding` for unsorted integer lists (using codec::dec and enc)
/// - `IncreasingEncoding` for increasing integer lists (using codec::ddec and denc)
/// - `StrictlyIncreasingEncoding` for strictly increasing integer lists (using codec::d1dec and d1enc)
/// - `ZigZagEncoding` for unsorted integer lists (using codec::zdec and zenc)
/// 
/// Along with those two function we have `dec_buf_len` and `enc_buf_size` that are used to calculate
/// the safe buffer sizes.
/// 
/// Each encoding gets an u8 identifier `ENC_TYPE` that can be used for lookup tables, etc.
/// 
/// The sample function is used to generate random data for testing the encodings.
use crate::codec::*;
use crate::sample::*;
use std::marker::PhantomData;
use rand::{ // for testing
    prelude::Distribution, 
    distributions::Standard,
};

/// Trait for encoding types with width.
pub trait Encoding {
    type W: Width;
    type T;
    /// Numeric ID useful for identifying the encoding type in lookup tables, etc.
    const ENC_TYPE: u8; 
    /// Encode input data into output buffer
    fn encode(input: &[Self::T], output: &mut [u8]) -> usize;
    /// Decode input buffer into output data
    fn decode(input: &[u8], n: usize,  output: &mut [Self::T]) -> usize;
    /// Minimum safe [T] array length required for decoding n T's
    fn dec_buf_len(n: usize) -> usize;
    /// Minimum safe u8 size required for encoding n T's
    fn enc_buf_size(n: usize) -> usize;
    /// Sample random data for Self::T suitable for testing this encoding
    fn sample(len: usize) -> Vec<Self::T>;
}

/// Unsorted integer lists (using codec::dec and enc)
pub struct StandardEncoding<WT, T>           { _marker_w: PhantomData<WT>, _marker_t: PhantomData<T> }

/// Increasing integer lists (using codec::ddec and denc)
pub struct IncreasingEncoding<WT, T>         { _marker_w: PhantomData<WT>, _marker_t: PhantomData<T> }

/// Strictly increasing integer lists (using codec::d1dec and d1enc)
pub struct StrictlyIncreasingEncoding<WT, T> { _marker_w: PhantomData<WT>, _marker_t: PhantomData<T> }

/// Unsorted integer lists (using codec::zdec and zenc)
pub struct ZigZagEncoding<WT, T>             { _marker_w: PhantomData<WT>, _marker_t: PhantomData<T> }

// Implement the Encoding trait for each encoding type, linking to the right Codec methods
impl<WT: Width, T: Codec<WT>> Encoding for StandardEncoding<WT, T> 
    where Standard: Distribution<T>, T: Arithmetic
{
    type W = WT;
    type T = T;
    const ENC_TYPE: u8 = 0; // Example encoding type constant
    fn encode(input: &[Self::T], output: &mut [u8]) -> usize {
        T::enc(input, output)
    }
    fn decode(input: &[u8], n: usize, output: &mut [Self::T]) -> usize {
        T::dec(input, n, output)
    }
    fn dec_buf_len(n: usize) -> usize { Self::W::dec_buf_len::<Self::T>(n) }
    fn enc_buf_size(n: usize) -> usize { Self::W::enc_buf_size::<Self::T>(n) }
    fn sample(len: usize) -> Vec<Self::T> { sample_standard(len) }
} 

impl<WT: Width, T: Codec<WT>> Encoding for IncreasingEncoding<WT, T> 
where Standard: Distribution<T>, T: Arithmetic
{
    type W = WT;
    type T = T;
    const ENC_TYPE: u8 = 1;
    fn encode(input: &[Self::T], output: &mut [u8]) -> usize {
        T::denc(input, output)
    }
    fn decode(input: &[u8], n: usize, output: &mut [Self::T]) -> usize {
        T::ddec(input, n, output)
    }
    fn dec_buf_len(n: usize) -> usize { Self::W::dec_buf_len::<Self::T>(n) }
    fn enc_buf_size(n: usize) -> usize { Self::W::enc_buf_size::<Self::T>(n) }
    fn sample(len: usize) -> Vec<Self::T>
        where Standard: Distribution<T>, T: Arithmetic
    { sample_increasing(len, 0, 10) }
}

impl<WT: Width, T: Codec<WT>> Encoding for StrictlyIncreasingEncoding<WT, T> 
    where Standard: Distribution<T>, T: Arithmetic
{
    type W = WT;
    type T = T;
    const ENC_TYPE: u8 = 2;
    fn encode(input: &[Self::T], output: &mut [u8]) -> usize {
        T::d1enc(input, output)
    }
    fn decode(input: &[u8], n: usize, output: &mut [Self::T]) -> usize {
        T::d1dec(input, n, output)
    }
    fn dec_buf_len(n: usize) -> usize { Self::W::dec_buf_len::<Self::T>(n) }
    fn enc_buf_size(n: usize) -> usize { Self::W::enc_buf_size::<Self::T>(n) }
    fn sample(len: usize) -> Vec<Self::T>
        where Standard: Distribution<T>, T: Arithmetic
    { sample_increasing(len, 1, 10) }
}

impl<WT: Width, T: Codec<WT>> Encoding for ZigZagEncoding<WT, T> 
    where Standard: Distribution<T>, T: Arithmetic
{
    type W = WT;
    type T = T;
    const ENC_TYPE: u8 = 3;
    fn encode(input: &[Self::T], output: &mut [u8]) -> usize {
        T::zenc(input, output)
    }
    fn decode(input: &[u8], n: usize, output: &mut [Self::T]) -> usize {
        T::zdec(input, n, output)
    }
    fn dec_buf_len(n: usize) -> usize { Self::W::dec_buf_len::<Self::T>(n) }
    fn enc_buf_size(n: usize) -> usize { Self::W::enc_buf_size::<Self::T>(n) }
    fn sample(len: usize) -> Vec<Self::T> { sample_standard(len) }
}

/// Generic encoding function
pub fn encode<E: Encoding>(input: &[E::T], output: &mut [u8]) -> usize {
    E::encode(input, output)
}

/// Generic decoding function
pub fn decode<E: Encoding>(input: &[u8], n: usize, output: &mut [E::T]) -> usize {
    E::decode(input, n, output)
}

#[cfg(test)]
use rand::Rng;

#[cfg(test)]
fn test_enc_dec_generic<E>()
where
    E: Encoding,
    E::T : Default + std::fmt::Debug,
    E::W : Width,
    Standard: Distribution<E::T>, E::T: Arithmetic
{
    let mut rng = rand::thread_rng();
    for _ in 0..256 {
        // Generate random input data
        let len = rng.gen_range(1..=16 * 1024);
        let input: Vec<E::T> = E::sample(len);
        // Prepare input buffer and encode
        let mut buf = vec![0u8; E::enc_buf_size(input.len())];
        let size_enc = encode::<E>(&input, &mut buf);
        // Prepare output buffer and decode
        let mut output: Vec<E::T> = vec![E::T::default(); E::dec_buf_len(input.len())];
        let size_dec = decode::<E>(&buf[..size_enc], input.len(), &mut output);
        // Check results
        assert_eq!(size_enc, size_dec);
        assert_eq!(input, output[..input.len()]);
    }
}

#[test]
fn test_enc_dec_u8_standard() {
    test_enc_dec_generic::<StandardEncoding<W, u8>>()
}

#[test]
fn test_enc_dec_u16_standard() {
    test_enc_dec_generic::<StandardEncoding<W, u16>>()
}

#[test]
fn test_enc_dec_u32_standard() {
    test_enc_dec_generic::<StandardEncoding<W, u32>>()
}

#[test]
fn test_enc_dec_u64_standard() {
    test_enc_dec_generic::<StandardEncoding<W, u64>>()
}

#[test]
fn test_enc_dec_u8_increasing() {
    test_enc_dec_generic::<IncreasingEncoding<W, u8>>()
}

#[test]
fn test_enc_dec_u16_increasing() {
    test_enc_dec_generic::<IncreasingEncoding<W, u16>>()
}

#[test]
fn test_enc_dec_u32_increasing() {
    test_enc_dec_generic::<IncreasingEncoding<W, u32>>()
}

#[test]
fn test_enc_dec_u64_increasing() {
    test_enc_dec_generic::<IncreasingEncoding<W, u64>>()
}

#[test]
fn test_enc_dec_u8_strictly_increasing() {
    test_enc_dec_generic::<StrictlyIncreasingEncoding<W, u8>>()
}

#[test]
fn test_enc_dec_u16_strictly_increasing() {
    test_enc_dec_generic::<StrictlyIncreasingEncoding<W, u16>>()
}

#[test]
fn test_enc_dec_u32_strictly_increasing() {
    test_enc_dec_generic::<StrictlyIncreasingEncoding<W, u32>>()
}

#[test]
fn test_enc_dec_u64_strictly_increasing() {
    test_enc_dec_generic::<StrictlyIncreasingEncoding<W, u64>>()
}

#[test]
fn test_enc_dec_u8_zigzag() {
    test_enc_dec_generic::<ZigZagEncoding<W, u8>>()
}

#[test]
fn test_enc_dec_u16_zigzag() {
    test_enc_dec_generic::<ZigZagEncoding<W, u16>>()
}

#[test]
fn test_enc_dec_u32_zigzag() {
    test_enc_dec_generic::<ZigZagEncoding<W, u32>>()
}

#[test]
fn test_enc_dec_u64_zigzag() {
    test_enc_dec_generic::<ZigZagEncoding<W, u64>>()
}