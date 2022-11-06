use super::codec::*;
use super::buffer::*;
use generate_random::GenerateRandom;
use std::fmt::Debug;
use rand::rngs::ThreadRng;
use rand::Rng;
use num::{Zero};


const MAX_TEST_LEN: usize = 1024 * 1024;
const N_ITERATIONS: usize = 128;

/// Compare two arrays
fn compare<T: Eq>(data1: &[T], data2: &[T]) -> bool {
    if data1.len() != data2.len() {
        println!("Length {} != {}", data1.len(), data2.len());
        return false
    }
    for i in 0..data1.len() {
        if data1[i] != data2[i] {
            return false;
        }
    }
    return true;
}

#[test]
fn basics_test() {
    const LEN: usize = 1024;
    let mut vec: Vec<u8> = vec![0; LEN];
    let mut buf: Buffer<u8> = Buffer::from(&mut vec);
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.as_mut_slice().len(), LEN);
}

#[test]
fn simple_test()
{
    // make data
    let mut v: Vec<u16> = vec![0; 512];
    for val in vec![92, 126, 114, 64, 173, 250, 75, 131, 40, 134, 173, 96, 30, 121, 25, 37, 238, 91, 94, 93, 158, 80, 101, 246, 71, 213, 43, 177, 144, 236, 129].iter() { v.push(*val); }
    
    // encode
    let mut vec: Vec<u8> = vec![0; 1024];
    let mut buf: Buffer<u8> = Buffer::from(&mut vec);
    let n_bytes_written = Codec::<W>::enc(&v, &mut buf);

    // reset buffer
    buf.reset();

    // decode
    let mut vec: Vec<u16> = vec![0; 1024];
    let mut output: Buffer<u16> = Buffer::from(&mut vec);
    let n_bytes_read = Codec::<W>::dec(&mut buf, v.len(), &mut output);

    // test
    assert_eq!(n_bytes_written, n_bytes_read);
    assert!(compare(&v, &output));
}

fn test_enc<W:Width<W>, T: GenerateRandom + Eq + Codec<W> + Debug + Zero + Copy>() {
    let mut rng = rand::thread_rng();
    let enc_size: usize = W::buf_size::<T>(MAX_TEST_LEN);
    let dec_size: usize = MAX_TEST_LEN+32;
    let mut vec_e: Vec<u8> = vec![0; enc_size];
    let mut encoded: Buffer<u8> = Buffer::from(&mut vec_e);
    let mut vec_d: Vec<T> = vec![T::zero(); dec_size];
    let mut decoded: Buffer<T> = Buffer::from(&mut vec_d);
    for _ in 0..N_ITERATIONS {
        // make data
        let len = rng.gen_range(1..MAX_TEST_LEN);   // length of randomly generated data
        let mut input: Vec<T> = vec![T::zero(); len+32+1_000_000];
        for _ in 0..len {
            input.push(T::generate_random(&mut rng))
        }
        // encode
        let n_bytes_written = T::enc(
            &input, 
            &mut encoded
        );
        assert_eq!(n_bytes_written, encoded.len());
        // reset buffer
        encoded.reset();
        assert_eq!(encoded.len(), 0);
        assert_eq!(input.len(), len);
        // decode
        let n_bytes_read = T::dec(
            &mut encoded,
            input.len(),
            &mut decoded
        );
        assert_eq!(n_bytes_read, encoded.len());
        assert_eq!(n_bytes_written, n_bytes_read);
        assert_eq!(len, decoded.len());
        assert!(compare(&input, &decoded));
        // Don't forget to truncate at iteration
        encoded.reset();
        assert!(encoded.len()==0);
        decoded.reset();
        assert!(decoded.len()==0);
    }
}

#[test]
fn test_enc8() {
    test_enc::<W,u8>();
}

#[test]
fn test_enc16() {
    test_enc::<W,u16>();
    test_enc::<W128v,u16>();
}

#[test]
fn test_enc32() {
    test_enc::<W,u32>();
    test_enc::<W128v,u32>();
}

#[test]
fn test_enc64() {
    test_enc::<W,u64>();
}

trait Num where Self: Sized {
    fn mk_data(len: usize, rng: &mut ThreadRng, positive:bool) -> Vec<Self>;
}

impl Num for u16 {
    fn mk_data(len: usize, rng: &mut ThreadRng, positive:bool) -> Vec<u16> {
        let mut input: Vec<u16> = vec![0; len+32];
        let mut x = 0;
        for _ in 0..len {
            x += rng.gen_range((if positive {1} else {0})..32);
            input.push(x);
        }        
        input
    }
}

impl Num for u32 {
    fn mk_data(len: usize, rng: &mut ThreadRng, positive:bool) -> Vec<u32> {
        let mut input: Vec<u32> = vec![0; len+32];
        let mut x = 0;
        for _ in 0..len {
            x += rng.gen_range((if positive {1} else {0})..256);
            input.push(x);
        }        
        input
    }
}

impl Num for u64 {
    fn mk_data(len: usize, rng: &mut ThreadRng, positive:bool) -> Vec<u64> {
        let mut input: Vec<u64> = vec![0; len+32];
        let mut x = 0;
        for _ in 0..len {
            x += rng.gen_range((if positive {1} else {0})..256*256);
            input.push(x);
        }        
        input
    }
}

fn test_generic<W:Width<W>, T: GenerateRandom + Eq + Codec<W> + Zero + Clone + Num>(
    max_test_len: usize,
    enc: fn(&Vec<T>, &mut Buffer<u8>) -> usize,
    dec: fn(&mut Buffer<u8>, usize, &mut Buffer<T>) -> usize
) 
{
    let mut rng = rand::thread_rng();
    let mut vec_e: Vec<u8> = vec![0; W::buf_size::<T>(max_test_len)];
    let mut encoded: Buffer<u8> = Buffer::from(&mut vec_e);
    let mut vec_d: Vec<T> = vec![T::zero(); W::buf_size::<T>(max_test_len+32)];
    let mut decoded: Buffer<T> = Buffer::from(&mut vec_d);
    for _ in 0..32 {
        let len = rng.gen_range(1..max_test_len);
        let input: Vec<T> = T::mk_data(len, &mut rng, false);
        let enc_size = enc(
            &input,
            &mut encoded
        );
        encoded.reset();
        let bytes_used = dec(
            &mut encoded,
            input.len(),
            &mut decoded
        );
        assert_eq!(enc_size, bytes_used);
        assert!(compare(&input, &decoded[..input.len()]));
        encoded.reset();
        decoded.reset();
    }
}

#[test]
fn test_denc16() {
    test_generic::<W, u16>(2047, Codec::<W>::denc, Codec::<W>::ddec);
    test_generic::<W128v, u16>(2047, Codec::<W128v>::denc, Codec::<W128v>::ddec);
}

#[test]
fn test_denc32() {
    test_generic::<W, u32>(MAX_TEST_LEN, Codec::<W>::denc, Codec::<W>::ddec);
    test_generic::<W128v, u32>(2047, Codec::<W128v>::denc, Codec::<W128v>::ddec);
    test_generic::<W256v, u32>(2047, Codec::<W256v>::denc, Codec::<W256v>::ddec);
}

#[test]
fn test_denc64() {
    test_generic::<W, u64>(MAX_TEST_LEN, Codec::<W>::denc, Codec::<W>::ddec);
}

#[test]
fn test_d1enc16() {
    test_generic::<W, u16>(2047, Codec::<W>::d1enc, Codec::<W>::d1dec);
    test_generic::<W128v, u16>(2047, Codec::<W128v>::d1enc, Codec::<W128v>::d1dec);
}

#[test]
fn test_d1enc32() {
    test_generic::<W, u32>(MAX_TEST_LEN, Codec::<W>::d1enc, Codec::<W>::d1dec);
}

#[test]
fn test_d1enc64() {
    test_generic::<W, u64>(MAX_TEST_LEN, Codec::<W>::d1enc, Codec::<W>::d1dec);
}

#[test]
fn test_zenc16() {
    test_generic::<W, u16>(2047, Codec::<W>::zenc, Codec::<W>::zdec);
    test_generic::<W128v, u16>(2047, Codec::<W128v>::zenc, Codec::<W128v>::zdec);
}

#[test]
fn test_zenc32() {
    test_generic::<W, u32>(MAX_TEST_LEN, Codec::<W>::zenc, Codec::<W>::zdec);
    test_generic::<W128v, u32>(2047, Codec::<W128v>::zenc, Codec::<W128v>::zdec);
    test_generic::<W256v, u32>(2047, Codec::<W256v>::zenc, Codec::<W256v>::zdec);
}

#[test]
fn test_zenc64() {
    test_generic::<W, u64>(MAX_TEST_LEN, Codec::<W>::zenc, Codec::<W>::zdec);
}
