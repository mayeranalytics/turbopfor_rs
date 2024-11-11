use std::mem::size_of;
use super::p4::*;

pub trait Width {
    /// Minimum safe [u8] size required for encoding `n` T's
    fn enc_buf_size<T>(n:usize) -> usize;
    /// Minimum safe [T] size required for decoding `n` T's
    fn dec_buf_len<T>(n:usize)  -> usize;
}

/// Default width
pub struct W;
impl Width for W {
    fn enc_buf_size<T>(n:usize) -> usize {
        (n+127)/128 + (n+32)*size_of::<T>()
    }
    fn dec_buf_len<T>(n:usize) -> usize { n+32 }
}

/// Width 128v
pub struct W128v;
impl Width for W128v {
    fn enc_buf_size<T>(n:usize) -> usize {
        (n+127)/128 + (n+32)*size_of::<T>()
    }
    fn dec_buf_len<T>(n:usize) -> usize { n+32 }
}

/// Width 256
pub struct W256v;
impl Width for W256v {
    fn enc_buf_size<T>(n:usize) -> usize {
        (n+255)/256 + (n+32)*size_of::<T>()
    }
    fn dec_buf_len<T>(n:usize) -> usize { n+32 }
}

pub trait Codec<W:Width> where Self:Sized {
    /// Turbopfor encoding for unsorted integer lists of type `Self`
    /// # Arguments
    /// * `input` - `&[T]` containing the uncompressed input
    /// * `output` - `[u8]` containing the compressed output
    /// # Returns
    /// Number of output bytes used
    fn enc(input: &[Self], output: &mut [u8]) -> usize;

    /// Turbopfor decoding into unsorted integer lists
    /// # Arguments
    /// * `input` - `&[u8]` containing the compressed input data
    /// * `n` - Length of decompressed data to be written to output
    /// * `output` - Output `&[T]` containing the decompressed data
    /// # Returns
    /// Number of input bytes used
    fn dec(input: &[u8], n: usize, output: &mut [Self]) -> usize;

    /// Turbopfor delta encoding for increasing integer lists of type `Self` (sorted w/ duplicate)
    /// # Arguments
    /// * `input` - `&[T]` containing the uncompressed input
    /// * `output` - `[u8]` containing the compressed output
    /// # Returns
    /// Number of output bytes used
    fn denc(input: &[Self], output: &mut [u8]) -> usize;

    /// Turbopfor delta decoding into increasing integer lists of type `Self` (sorted w/ duplicate)
    /// # Arguments
    /// * `input` - `&[u8]` containing the compressed input data
    /// * `n` - Length of decompressed data to be written to output
    /// * `output` - Output `&[T]` containing the decompressed data
    /// # Returns
    /// Number of input bytes used
    fn ddec(input: &[u8], n: usize, output: &mut [Self]) -> usize;

    /// Turbopfor delta encoding for strictly increasing integer lists of type `Self` (sorted unique)
    /// # Arguments
    /// * `input` - `&[T]` containing the uncompressed input
    /// * `output` - `[u8]` containing the compressed output
    /// # Returns
    /// Number of output bytes used
    fn d1enc(input: &[Self], output: &mut [u8]) -> usize;

    /// Turbopfor delta decoding into strictly increasing integer lists of type `Self` (sorted unique)
    /// # Arguments
    /// * `input` - `&[u8]` containing the compressed input data
    /// * `n` - Length of decompressed data to be written to output
    /// * `output` - Output `&[T]` containing the decompressed data
    /// # Returns
    /// Number of input bytes used
    fn d1dec(input: &[u8], n: usize, output: &mut [Self]) -> usize;

    /// Turbopfor dZigZag encoding for unsorted integer lists of type `Self`
    /// # Arguments
    /// * `input` - `&[T]` containing the uncompressed input
    /// * `output` - `[u8]` containing the compressed output
    /// # Returns
    /// Number of output bytes used
    fn zenc(input: &[Self], output: &mut [u8]) -> usize;

    /// Turbopfor ZigZag decoding for unsorted integer lists of type `Self`
    /// # Arguments
    /// * `input` - `&[u8]` containing the compressed input data
    /// * `n` - Length of decompressed data to be written to output
    /// * `output` - Output `&[T]` containing the decompressed data
    /// # Returns
    /// Number of input bytes used
    fn zdec(input: &[u8], n: usize, output: &mut [Self]) -> usize;

}

impl Codec<W> for u8 {
    fn enc(input: &[u8], output: &mut [u8]) -> usize {
        enc8(input, output)
    }
    fn dec(input: &[u8], n: usize, output: &mut [u8]) -> usize {
        dec8(input, n, output)
    }
    fn denc(input: &[u8], output: &mut [u8]) -> usize {
        denc8(input, output)
    }
    fn ddec(input: &[u8], n: usize, output: &mut [u8]) -> usize {
        ddec8(input, n, output)
    }
    fn d1enc(input: &[u8], output: &mut [u8]) -> usize {
        d1enc8(input, output)
    }
    fn d1dec(input: &[u8], n: usize, output: &mut [u8]) -> usize {
        d1dec8(input, n, output)
    }
    fn zenc(input: &[u8], output: &mut [u8]) -> usize {
        zenc8(input, output)
    }
    fn zdec(input: &[u8], n: usize, output: &mut [u8]) -> usize {
        zdec8(input, n, output)
    }
}

impl Codec<W> for u16 {
    fn enc(input: &[u16], output: &mut [u8]) -> usize {
        enc16(input, output)
    }
    fn dec(input: &[u8], n: usize, output: &mut [u16]) -> usize {
        dec16(input, n, output)
    }
    fn denc(input: &[u16], output: &mut [u8]) -> usize {
        denc16(input, output)
    }
    fn ddec(input: &[u8], n: usize, output: &mut [u16]) -> usize {
        ddec16(input, n, output)
    }
    fn d1enc(input: &[u16], output: &mut [u8]) -> usize {
        d1enc16(input, output)
    }
    fn d1dec(input: &[u8], n: usize, output: &mut [u16]) -> usize {
        d1dec16(input, n, output)
    }
    fn zenc(input: &[u16], output: &mut [u8]) -> usize {
        zenc16(input, output)
    }
    fn zdec(input: &[u8], n: usize, output: &mut [u16]) -> usize {
        zdec16(input, n, output)
    }
}

impl Codec<W> for u32 {
    fn enc(input: &[u32], output: &mut [u8]) -> usize {
        enc32(input, output)
    }
    fn dec(input: &[u8], n: usize, output: &mut [u32]) -> usize {
        dec32(input, n, output)
    }
    fn denc(input: &[u32], output: &mut [u8]) -> usize {
        denc32(input, output)
    }
    fn ddec(input: &[u8], n: usize, output: &mut [u32]) -> usize {
        ddec32(input, n, output)
    }
    fn d1enc(input: &[u32], output: &mut [u8]) -> usize {
        d1enc32(input, output)
    }
    fn d1dec(input: &[u8], n: usize, output: &mut [u32]) -> usize {
        d1dec32(input, n, output)
    }
    fn zenc(input: &[u32], output: &mut [u8]) -> usize {
        zenc32(input, output)
    }
    fn zdec(input: &[u8], n: usize, output: &mut [u32]) -> usize {
        zdec32(input, n, output)
    }
}

impl Codec<W> for u64 {
    fn enc(input: &[u64], output: &mut [u8]) -> usize {
        enc64(input, output)
    }
    fn dec(input: &[u8], n: usize, output: &mut [u64]) -> usize {
        dec64(input, n, output)
    }
    fn denc(input: &[u64], output: &mut [u8]) -> usize {
        denc64(input, output)
    }
    fn ddec(input: &[u8], n: usize, output: &mut [u64]) -> usize {
        ddec64(input, n, output)
    }
    fn d1enc(input: &[u64], output: &mut [u8]) -> usize {
        d1enc64(input, output)
    }
    fn d1dec(input: &[u8], n: usize, output: &mut [u64]) -> usize {
        d1dec64(input, n, output)
    }
    fn zenc(input: &[u64], output: &mut [u8]) -> usize {
        zenc64(input, output)
    }
    fn zdec(input: &[u8], n: usize, output: &mut [u64]) -> usize {
        zdec64(input, n, output)
    }
}

impl Codec<W128v> for u16 {
    fn enc(input: &[u16], output: &mut [u8]) -> usize {
        enc128v16(input, output)
    }
    fn dec(input: &[u8], n: usize, output: &mut [u16]) -> usize {
        dec128v16(input, n, output)
    }
    fn denc(input: &[u16], output: &mut [u8]) -> usize {
        denc128v16(input, output)
    }
    fn ddec(input: &[u8], n: usize, output: &mut [u16]) -> usize {
        ddec128v16(input, n, output)
    }
    fn d1enc(input: &[u16], output: &mut [u8]) -> usize {
        d1enc128v16(input, output)
    }
    fn d1dec(input: &[u8], n: usize, output: &mut [u16]) -> usize {
        d1dec128v16(input, n, output)
    }
    fn zenc(input: &[u16], output: &mut [u8]) -> usize {
        zenc128v16(input, output)
    }
    fn zdec(input: &[u8], n: usize, output: &mut [u16]) -> usize {
        zdec128v16(input, n, output)
    }
}

impl Codec<W128v> for u32 {
    fn enc(input: &[u32], output: &mut [u8]) -> usize {
        enc128v32(input, output)
    }
    fn dec(input: &[u8], n: usize, output: &mut [u32]) -> usize {
        dec128v32(input, n, output)
    }
    fn denc(input: &[u32], output: &mut [u8]) -> usize {
        denc128v32(input, output)
    }
    fn ddec(input: &[u8], n: usize, output: &mut [u32]) -> usize {
        ddec128v32(input, n, output)
    }
    fn d1enc(input: &[u32], output: &mut [u8]) -> usize {
        d1enc128v32(input, output)
    }
    fn d1dec(input: &[u8], n: usize, output: &mut [u32]) -> usize {
        d1dec128v32(input, n, output)
    }
    fn zenc(input: &[u32], output: &mut [u8]) -> usize {
        zenc128v32(input, output)
    }
    fn zdec(input: &[u8], n: usize, output: &mut [u32]) -> usize {
        zdec128v32(input, n, output)
    }
}

impl Codec<W256v> for u32 {
    fn enc(input: &[u32], output: &mut [u8]) -> usize {
        enc256v32(input, output)
    }
    fn dec(input: &[u8], n: usize, output: &mut [u32]) -> usize {
        dec256v32(input, n, output)
    }
    fn denc(input: &[u32], output: &mut [u8]) -> usize {
        denc256v32(input, output)
    }
    fn ddec(input: &[u8], n: usize, output: &mut [u32]) -> usize {
        ddec256v32(input, n, output)
    }
    fn d1enc(input: &[u32], output: &mut [u8]) -> usize {
        d1enc256v32(input, output)
    }
    fn d1dec(input: &[u8], n: usize, output: &mut [u32]) -> usize {
        d1dec256v32(input, n, output)
    }
    fn zenc(input: &[u32], output: &mut [u8]) -> usize {
        zenc256v32(input, output)
    }
    fn zdec(input: &[u8], n: usize, output: &mut [u32]) -> usize {
        zdec256v32(input, n, output)
    }
}
