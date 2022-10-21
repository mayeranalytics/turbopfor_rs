use std::mem::size_of;
use super::p4::*;
use super::buffer::*;

pub trait Width<W> {
    /// Maximum encoded size required
    fn buf_size<T>(n:usize)  -> usize;
}

/// Default width
pub struct W;
impl Width<W> for W {
    fn buf_size<T>(n:usize)  -> usize {
        (n+127)/128 + (n+32)*size_of::<T>()
    }
}

/// Width 128v
pub struct W128v;
impl Width<W128v> for W128v {
    fn buf_size<T>(n:usize)  -> usize {
        (n+127)/128 + (n+32)*size_of::<T>()
    }
}

/// Width 256
pub struct W256v;
impl Width<W256v> for W256v {
    fn buf_size<T>(n:usize)  -> usize {
        (n+255)/256 + (n+32)*size_of::<T>()
    }
}

pub trait Codec<W:Width<W>> where Self:Sized {
    /// Turbopfor encoding for unsorted integer lists of type `T`
    /// # Arguments
    /// * `input` - `Vec<T>` containing the uncompressed input
    /// * `output` - `Buffer<u8>` containing the compressed output
    /// # Returns
    /// Number of output bytes used
    fn enc(input: &Vec<Self>, output: &mut Buffer<u8>) -> usize;

    /// Turbopfor decoding into unsorted integer lists
    /// # Arguments
    /// * `input` - `Vec<u8>` containing the compressed input data
    /// * `n` - Length of decompressed data to be written to output
    /// * `output` - Output `Vec<T>` containing the decompressed data
    /// # Returns
    /// Number of input bytes used
    fn dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<Self>) -> usize;

    /// Turbopfor delta encoding for increasing integer lists of type `T` (sorted w/ duplicate)
    /// # Arguments
    /// * `input` - `Vec<T>` containing the uncompressed input
    /// * `output` - `Buffer<u8>` containing the compressed output
    /// # Returns
    /// Number of output bytes used
    fn denc(input: &Vec<Self>, output: &mut Buffer<u8>) -> usize;

    /// Turbopfor delta decoding into increasing integer lists of type `T` (sorted w/ duplicate)
    /// # Arguments
    /// * `input` - `Vec<u8>` containing the compressed input data
    /// * `n` - Length of decompressed data to be written to output
    /// * `output` - Output `Vec<T>` containing the decompressed data
    /// # Returns
    /// Number of input bytes used
    fn ddec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<Self>) -> usize;

    /// Turbopfor delta encoding for strictly increasing integer lists of type `T` (sorted unique)
    /// # Arguments
    /// * `input` - `Vec<T>` containing the uncompressed input
    /// * `output` - `Buffer<u8>` containing the compressed output
    /// # Returns
    /// Number of output bytes used
    fn d1enc(input: &Vec<Self>, output: &mut Buffer<u8>) -> usize;

    /// Turbopfor delta decoding into strictly increasing integer lists of type `T` (sorted unique)
    /// # Arguments
    /// * `input` - `Vec<u8>` containing the compressed input data
    /// * `n` - Length of decompressed data to be written to output
    /// * `output` - Output `Vec<T>` containing the decompressed data
    /// # Returns
    /// Number of input bytes used
    fn d1dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<Self>) -> usize;

    /// Turbopfor dZigZag encoding for unsorted integer lists of type `T`
    /// # Arguments
    /// * `input` - `Vec<T>` containing the uncompressed input
    /// * `output` - `Buffer<u8>` containing the compressed output
    /// # Returns
    /// Number of output bytes used
    fn zenc(input: &Vec<Self>, output: &mut Buffer<u8>) -> usize;

    /// Turbopfor ZigZag decoding for unsorted integer lists of type `T`
    /// # Arguments
    /// * `input` - `Vec<u8>` containing the compressed input data
    /// * `n` - Length of decompressed data to be written to output
    /// * `output` - Output `Vec<T>` containing the decompressed data
    /// # Returns
    /// Number of input bytes used
    fn zdec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<Self>) -> usize;

}

impl Codec<W> for u8 {
    fn enc(input: &Vec<u8>, output: &mut Buffer<u8>) -> usize {
        enc8(input, output)
    }
    fn dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u8>) -> usize {
        dec8(input, n, output)
    }
    fn denc(input: &Vec<u8>, output: &mut Buffer<u8>) -> usize {
        denc8(input, output)
    }
    fn ddec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u8>) -> usize {
        ddec8(input, n, output)
    }
    fn d1enc(input: &Vec<u8>, output: &mut Buffer<u8>) -> usize {
        d1enc8(input, output)
    }
    fn d1dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u8>) -> usize {
        d1dec8(input, n, output)
    }
    fn zenc(input: &Vec<u8>, output: &mut Buffer<u8>) -> usize {
        zenc8(input, output)
    }
    fn zdec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u8>) -> usize {
        zdec8(input, n, output)
    }
}

impl Codec<W> for u16 {
    fn enc(input: &Vec<u16>, output: &mut Buffer<u8>) -> usize {
        enc16(input, output)
    }
    fn dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u16>) -> usize {
        dec16(input, n, output)
    }
    fn denc(input: &Vec<u16>, output: &mut Buffer<u8>) -> usize {
        denc16(input, output)
    }
    fn ddec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u16>) -> usize {
        ddec16(input, n, output)
    }
    fn d1enc(input: &Vec<u16>, output: &mut Buffer<u8>) -> usize {
        d1enc16(input, output)
    }
    fn d1dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u16>) -> usize {
        d1dec16(input, n, output)
    }
    fn zenc(input: &Vec<u16>, output: &mut Buffer<u8>) -> usize {
        zenc16(input, output)
    }
    fn zdec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u16>) -> usize {
        zdec16(input, n, output)
    }
}

impl Codec<W> for u32 {
    fn enc(input: &Vec<u32>, output: &mut Buffer<u8>) -> usize {
        enc32(input, output)
    }
    fn dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u32>) -> usize {
        dec32(input, n, output)
    }
    fn denc(input: &Vec<u32>, output: &mut Buffer<u8>) -> usize {
        denc32(input, output)
    }
    fn ddec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u32>) -> usize {
        ddec32(input, n, output)
    }
    fn d1enc(input: &Vec<u32>, output: &mut Buffer<u8>) -> usize {
        d1enc32(input, output)
    }
    fn d1dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u32>) -> usize {
        d1dec32(input, n, output)
    }
    fn zenc(input: &Vec<u32>, output: &mut Buffer<u8>) -> usize {
        zenc32(input, output)
    }
    fn zdec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u32>) -> usize {
        zdec32(input, n, output)
    }
}

impl Codec<W> for u64 {
    fn enc(input: &Vec<u64>, output: &mut Buffer<u8>) -> usize {
        enc64(input, output)
    }
    fn dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u64>) -> usize {
        dec64(input, n, output)
    }
    fn denc(input: &Vec<u64>, output: &mut Buffer<u8>) -> usize {
        denc64(input, output)
    }
    fn ddec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u64>) -> usize {
        ddec64(input, n, output)
    }
    fn d1enc(input: &Vec<u64>, output: &mut Buffer<u8>) -> usize {
        d1enc64(input, output)
    }
    fn d1dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u64>) -> usize {
        d1dec64(input, n, output)
    }
    fn zenc(input: &Vec<u64>, output: &mut Buffer<u8>) -> usize {
        zenc64(input, output)
    }
    fn zdec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u64>) -> usize {
        zdec64(input, n, output)
    }
}

impl Codec<W128v> for u16 {
    fn enc(input: &Vec<u16>, output: &mut Buffer<u8>) -> usize {
        enc128v16(input, output)
    }
    fn dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u16>) -> usize {
        dec128v16(input, n, output)
    }
    fn denc(input: &Vec<u16>, output: &mut Buffer<u8>) -> usize {
        denc128v16(input, output)
    }
    fn ddec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u16>) -> usize {
        ddec128v16(input, n, output)
    }
    fn d1enc(input: &Vec<u16>, output: &mut Buffer<u8>) -> usize {
        d1enc128v16(input, output)
    }
    fn d1dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u16>) -> usize {
        d1dec128v16(input, n, output)
    }
    fn zenc(input: &Vec<u16>, output: &mut Buffer<u8>) -> usize {
        zenc128v16(input, output)
    }
    fn zdec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u16>) -> usize {
        zdec128v16(input, n, output)
    }
}

impl Codec<W128v> for u32 {
    fn enc(input: &Vec<u32>, output: &mut Buffer<u8>) -> usize {
        enc128v32(input, output)
    }
    fn dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u32>) -> usize {
        dec128v32(input, n, output)
    }
    fn denc(input: &Vec<u32>, output: &mut Buffer<u8>) -> usize {
        denc128v32(input, output)
    }
    fn ddec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u32>) -> usize {
        ddec128v32(input, n, output)
    }
    fn d1enc(input: &Vec<u32>, output: &mut Buffer<u8>) -> usize {
        d1enc128v32(input, output)
    }
    fn d1dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u32>) -> usize {
        d1dec128v32(input, n, output)
    }
    fn zenc(input: &Vec<u32>, output: &mut Buffer<u8>) -> usize {
        zenc128v32(input, output)
    }
    fn zdec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u32>) -> usize {
        zdec128v32(input, n, output)
    }
}

impl Codec<W256v> for u32 {
    fn enc(input: &Vec<u32>, output: &mut Buffer<u8>) -> usize {
        enc256v32(input, output)
    }
    fn dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u32>) -> usize {
        dec256v32(input, n, output)
    }
    fn denc(input: &Vec<u32>, output: &mut Buffer<u8>) -> usize {
        denc256v32(input, output)
    }
    fn ddec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u32>) -> usize {
        ddec256v32(input, n, output)
    }
    fn d1enc(input: &Vec<u32>, output: &mut Buffer<u8>) -> usize {
        d1enc256v32(input, output)
    }
    fn d1dec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u32>) -> usize {
        d1dec256v32(input, n, output)
    }
    fn zenc(input: &Vec<u32>, output: &mut Buffer<u8>) -> usize {
        zenc256v32(input, output)
    }
    fn zdec(input: &mut Buffer<u8>, n: usize, output: &mut Buffer<u32>) -> usize {
        zdec256v32(input, n, output)
    }
}
