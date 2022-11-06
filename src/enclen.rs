use generate_random::GenerateRandom;
use turbopfor_rs::codec::*;
use turbopfor_rs::buffer::*;

fn enc_len<W:Width<W>, T: GenerateRandom + Eq + Codec<W>>(len: usize, n_iter: usize) -> usize {
    let mut rng = rand::thread_rng();
    let mut vec: Vec<u8> = Vec::with_capacity(500_000_000);
    let mut encoded: Buffer<u8> = Buffer::from(&mut vec);
    let mut max_enc_size: usize = 0;
    for _ in 0..n_iter {
        let mut input: Vec<T> = Vec::with_capacity(len+32);
        for _ in 0..len {
            input.push(T::generate_random(&mut rng));
        }
        let enc_size = T::enc(&input, &mut encoded);
        if enc_size > max_enc_size { max_enc_size = enc_size; }
    }
    max_enc_size
}

fn main() {
    println!("len\tu8\tu16\tu32\tu16 128v\tu32 128v\tu32 256v");
    for len in 1..10000 {
        //let len: usize = l * 10;
        let enc_l_8 = enc_len::<W,u8>(len, 1000);
        let enc_l_16 = enc_len::<W,u16>(len, 1000);
        let enc_l_32 = enc_len::<W,u32>(len, 1000);
        let enc_l_16_128v = enc_len::<W128v,u16>(len, 1000);
        let enc_l_32_128v = enc_len::<W128v,u32>(len, 1000);
        let enc_l_32_256v = enc_len::<W256v,u32>(len, 1000);
        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}", len, enc_l_8-len, enc_l_16-len*2, enc_l_32-len*4, enc_l_16_128v-len*2, enc_l_32_128v-len*4, enc_l_32_256v-len*4);
    }
}
