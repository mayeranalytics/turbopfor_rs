use super::codec::*;
use std::{
    fmt::Debug,
    collections::HashSet, hash::Hash,
};
use rand::{
    rngs::ThreadRng,
    Rng, 
};

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
fn simple_test()
{
    // make data
    let v: Vec<u16> = vec![92, 126, 114, 64, 173, 250, 75, 131, 40, 134, 173, 96, 30, 121, 25, 37, 238, 91, 94, 93, 158, 80, 101, 246, 71, 213, 43, 177, 144, 236, 129];

    // encode
    let mut buf: Vec<u8> = vec![0; 1024];
    let n_bytes_written = Codec::<W>::enc(&v, &mut buf);

    // decode
    let mut output: Vec<u16> = vec![0; 1024];
    let n_bytes_read = Codec::<W>::dec(&buf, v.len(), &mut output);

    // test
    assert_eq!(n_bytes_written, n_bytes_read);
    assert!(compare(&v, &output[..v.len()]));
}

#[derive(PartialEq, Eq)]
enum DataType {
    Increasing,
    StrictlyIncreasing,
    Unsorted
}

trait Num where Self: Sized {
    fn zero() -> Self;
    fn upper_bound() -> usize;
    fn rand(rng: &mut ThreadRng) -> Self;
}

impl Num for u8 {
    fn zero() -> u8 { 0 }
    fn upper_bound() -> usize { 255 }
    fn rand(rng: &mut ThreadRng) -> u8 { rng.gen() }
}

impl Num for u16 {
    fn zero() -> u16 { 0 }
    fn upper_bound() -> usize { 65535 }
    fn rand(rng: &mut ThreadRng) -> u16 { rng.gen() }
}

impl Num for u32 {
    fn zero() -> u32 { 0 }
    fn upper_bound() -> usize { 4294967295 }
    fn rand(rng: &mut ThreadRng) -> u32 { rng.gen() }
}

impl Num for u64 {
    fn zero() -> u64 { 0 }
    fn upper_bound() -> usize { 18446744073709551615 } // you better be on a 64 bit system
    fn rand(rng: &mut ThreadRng) -> u64 { rng.gen() }
}

/// Make increasing data of maximum length `max_len`. If `strictly` is true, make strictly increasing data.
fn mk_data_inc<T: Num+Copy+std::ops::AddAssign+Hash+Eq+Ord>(
    max_len: usize,
    rng: &mut ThreadRng,
    data_type: &DataType
) -> Vec<T>
{
    match data_type {
        DataType::StrictlyIncreasing => {
            let len = rng.gen_range(1..std::cmp::min(max_len, T::upper_bound()));
            let mut data: HashSet<T> = HashSet::new();
            for _ in 0..len { data.insert(T::rand(rng)); }
            let mut data: Vec<T> = data.into_iter().collect();
            data.sort();
            data
        },
        DataType::Increasing => {
            let len = rng.gen_range(1..max_len);
            let mut data: Vec<T> = Vec::with_capacity(MAX_TEST_LEN);
            for _ in 0..len { data.push(T::rand(rng)); }
            data.sort();
            data
        },
        DataType::Unsorted => {
            let len = rng.gen_range(1..max_len);
            let mut data: Vec<T> = Vec::with_capacity(MAX_TEST_LEN);
            for _ in 0..len { data.push(T::rand(rng)); }
            data
        }
    }
}


/// Checks that no data was written beyond `len`, i.e. returns true if
/// data[len], data[len+1], ... are all zero.
fn check_no_overflow<T: Ord+Debug+Num>(data: &[T], len: usize) -> bool {
    if len > data.len() { return true; }
    if len == 0 || data.is_empty() { return false; }
    let mut i = data.len();
    while i > 0 {
        i -= 1;
        if data[i] != T::zero() { break; }
    }
    if i > len-1 {
        println!("empty data starts at index {}, {} positions after len index={}", i, i-len+1, len-1);
        let m = std::cmp::min(len+128, data.len());
        println!("{:?}", &data[len..m]);
        false
    } else {
        true
    }
}

#[test]
fn test_check_no_overflow()
{
    let input = [1u8, 2, 3, 0, 0];
    assert!(check_no_overflow(&input, 0)==false);
    assert!(check_no_overflow(&input, 1)==false);
    assert!(check_no_overflow(&input, 2)==false);
    assert!(check_no_overflow(&input, 3)==true);
    assert!(check_no_overflow(&input, 4)==true);
    assert!(check_no_overflow(&input, 5)==true);
    assert!(check_no_overflow(&input, 6)==true);
}

fn test_generic<W:Width, T: Num+Copy+std::ops::AddAssign+Hash+Eq+Ord+Debug>(
    max_test_len: usize,
    enc: fn(&[T], &mut [u8]) -> usize,
    dec: fn(&[u8], usize, &mut [T]) -> usize,
    data_type: DataType
) 
{
    let mut rng = rand::thread_rng();

    // alloc
    let enc_size: usize = W::buf_size::<T>(max_test_len);
    let dec_size: usize = max_test_len+32;
    let mut encoded: Vec<u8> = vec![0; enc_size];
    let mut decoded: Vec<T> = vec![T::zero(); dec_size];

    // repeat test
    for _ in 0..N_ITERATIONS {
        // make data
        let len = rng.gen_range(1..max_test_len);   // length of randomly generated input data
        let mut input: Vec<T> = mk_data_inc(len, &mut rng, &data_type);
        while input.len() < max_test_len { input.push(T::zero()); }
        // encode
        let n_bytes_written = enc(
            &input, 
            &mut encoded
        );
        assert!(check_no_overflow(&encoded, n_bytes_written));
        // decode
        let n_bytes_read = dec(
            &mut encoded,
            input.len(),
            &mut decoded
        );
        assert_eq!(n_bytes_written, n_bytes_read);
        assert!(compare(&input, &decoded[..input.len()]));
        assert!(check_no_overflow(&decoded, len));
        // Reset buffers
        for i in 0..encoded.len() { encoded[i] = 0; }
        for i in 0..decoded.len() { decoded[i] = T::zero(); }
    }

    // test with tight allocations
    for _ in 0..N_ITERATIONS/8 {
        let len = rng.gen_range(1..max_test_len);   // length of randomly generated input data
        let enc_size: usize = W::buf_size::<T>(len);
        let dec_size: usize = len+32;
        let mut encoded: Vec<u8> = vec![0; enc_size];
        let mut decoded: Vec<T> = vec![T::zero(); dec_size];
        // make data
        let mut input: Vec<T> = mk_data_inc(len, &mut rng, &data_type);
        while input.len() < len { input.push(T::zero()); }
        // encode
        let n_bytes_written = enc(
            &input, 
            &mut encoded
        );
        assert!(check_no_overflow(&encoded, n_bytes_written));
        // decode
        let n_bytes_read = dec(
            &mut encoded,
            input.len(),
            &mut decoded
        );
        assert_eq!(n_bytes_written, n_bytes_read);
        assert!(compare(&input, &decoded[..input.len()]));
        assert!(check_no_overflow(&decoded, len));
        // Reset buffers
        for i in 0..encoded.len() { encoded[i] = 0; }
        for i in 0..decoded.len() { decoded[i] = T::zero(); }
    }
}


//////////////////////////////////////// enc ////////////////////////////////////////

#[test]
fn test_enc8() {
    test_generic::<W, u8>(MAX_TEST_LEN, Codec::<W>::enc,     Codec::<W>::dec    , DataType::Unsorted);
}

#[test]
fn test_enc16() {
    test_generic::<W,     u16>(MAX_TEST_LEN, Codec::<W>::enc, Codec::<W>::dec, DataType::Unsorted);
    test_generic::<W128v, u16>(MAX_TEST_LEN, Codec::<W>::enc, Codec::<W>::dec, DataType::Unsorted);
}

#[test]
fn test_enc32() {
    test_generic::<W,     u32>(MAX_TEST_LEN, Codec::<W>::enc, Codec::<W>::dec, DataType::Unsorted);
    test_generic::<W128v, u32>(MAX_TEST_LEN, Codec::<W>::enc, Codec::<W>::dec, DataType::Unsorted);
}

#[test]
fn test_enc64() {
    test_generic::<W, u64>(MAX_TEST_LEN, Codec::<W>::enc, Codec::<W>::dec, DataType::Unsorted);
}

//////////////////////////////////////// denc ////////////////////////////////////////

#[test]
fn test_denc16() {
    test_generic::<W,     u16>(MAX_TEST_LEN, Codec::<W>::denc,     Codec::<W>::ddec    , DataType::Increasing);
    test_generic::<W128v, u16>(MAX_TEST_LEN, Codec::<W128v>::denc, Codec::<W128v>::ddec, DataType::Increasing);
}

#[test]
fn test_denc32() {
    test_generic::<W,     u32>(MAX_TEST_LEN, Codec::<W>::denc,     Codec::<W>::ddec    , DataType::Increasing);
    test_generic::<W128v, u32>(MAX_TEST_LEN, Codec::<W128v>::denc, Codec::<W128v>::ddec, DataType::Increasing);
    test_generic::<W256v, u32>(MAX_TEST_LEN, Codec::<W256v>::denc, Codec::<W256v>::ddec, DataType::Increasing);
}

#[test]
fn test_denc64() {
    test_generic::<W, u64>(MAX_TEST_LEN, Codec::<W>::denc, Codec::<W>::ddec, DataType::Increasing);
}

//////////////////////////////////////// d1enc ////////////////////////////////////////

#[test]
fn test_d1enc16() {
    test_generic::<W,     u16>(MAX_TEST_LEN, Codec::<W>::d1enc,     Codec::<W>::d1dec,     DataType::StrictlyIncreasing);
    test_generic::<W128v, u16>(MAX_TEST_LEN, Codec::<W128v>::d1enc, Codec::<W128v>::d1dec, DataType::StrictlyIncreasing);
}

#[test]
fn test_d1enc32() {
    test_generic::<W, u32>(MAX_TEST_LEN, Codec::<W>::d1enc, Codec::<W>::d1dec, DataType::StrictlyIncreasing);
}

#[test]
fn test_d1enc64() {
    test_generic::<W, u64>(MAX_TEST_LEN, Codec::<W>::d1enc, Codec::<W>::d1dec, DataType::StrictlyIncreasing);
}

//////////////////////////////////////// zenc ////////////////////////////////////////

#[test]
fn test_zenc16() {
    test_generic::<W,     u16>(MAX_TEST_LEN, Codec::<W>::zenc,     Codec::<W>::zdec    , DataType::Unsorted);
    test_generic::<W128v, u16>(MAX_TEST_LEN, Codec::<W128v>::zenc, Codec::<W128v>::zdec, DataType::Unsorted);
}

#[test]
fn test_zenc32() {
    test_generic::<W,     u32>(MAX_TEST_LEN, Codec::<W>::zenc,     Codec::<W>::zdec    , DataType::Unsorted);
    test_generic::<W128v, u32>(MAX_TEST_LEN, Codec::<W128v>::zenc, Codec::<W128v>::zdec, DataType::Unsorted);
    test_generic::<W256v, u32>(MAX_TEST_LEN, Codec::<W256v>::zenc, Codec::<W256v>::zdec, DataType::Unsorted);
}

#[test]
fn test_zenc64() {
    test_generic::<W, u64>(MAX_TEST_LEN, Codec::<W>::zenc, Codec::<W>::zdec, DataType::Unsorted);
}
