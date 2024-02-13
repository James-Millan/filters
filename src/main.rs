mod bloomfilter;
mod cuckoofilter;
mod countingbloomfilter;
mod xorfilter;
mod bitvector;
mod blockedbloomfilter;
mod utils;
mod dleftcountingfilter;
mod registeralignedbloomfilter;
mod threewisebinaryfusefilter32;
mod simdblockedbloomfilter;
mod fourwisebinaryfusefilter;
mod threewisebinaryfusefilter16;
mod threewisebinaryfusefilter8;
mod fpr;
mod XorFilter8;

extern crate rand;
#[feature(core)]
use std::simd::f32x4;



use rand::Rng;
use rand::seq::index::sample;
use simd::f32x4;
use crate::fpr::bloom_filter_fpr;

use crate::utils::{hash, perfect_hashing};


fn main() {
    // let sample_sizes: Vec<u64> = vec![10,100,1000,10000,100000,1000000,10000000,100000000,1000000000,10000000000,100000000000,
    // 1000000000000,10000000000000,100000000000000,1000000000000000,10000000000000000,100000000000000000,1000000000000000000,
    // 10000000000000000000];
    // for size in sample_sizes {
    //     println!("{}", size);
    //     let mut keys = Vec::new();
    //     for i in 0..=size {
    //         keys.push(i);
    //     }
    //     fpr::blocked_bloom_filter_fpr(size,0.01,&keys);
    // }

    let mut keys = Vec::new();
    for i in 0..=10000 {
        keys.push(i as u64);
    }

    // create simd vectors
    let x = f32x4(1.0, 2.0, 3.0, 4.0);
    let y = f32x4(4.0, 3.0, 2.0, 1.0);

    // simd product
    let z = x * y;

    // like any struct, the simd vector can be destructured using `let`
    let f32x4(a, b, c, d) = z;

    println!("{:?}", (a, b, c, d));

    // let mut simdBloom = simdblockedbloomfilter::SimdBlockedBloomFilter::new(keys.len() as u64, 64, 0.01);
    // for key in keys {
    //     simdBloom.insert(key);
    // }
    //
    // for key in keys {
    //     println!("{} {}", key, simdBloom.member(key));
    // }
    //
    // fpr::bloom_filter_fpr(sample_size, 0.01, &keys);
    // fpr::counting_bloom_filter_fpr(sample_size,0.01,&keys);
    // fpr::cuckoo_filter_fpr(sample_size, 0.01, &keys);

    // fpr::binary_fuse_filter_8_fpr(&keys);
    // fpr::binary_fuse_filter_fpr(&keys);
    // fpr::blocked_bloom_filter_fpr(sample_size,0.01,&keys);
    // fpr::register_aligned_bloom_filter_fpr(sample_size,0.01,&keys);
    // //let xorfilter = xorfilter::XorFilter::new(keys);
    // //let perfect = perfect_hashing(&keys);
    // let binaryfusefilter = threewisebinaryfusefilter32::ThreeWiseBinaryFuseFilter32::new(keys);
    //
    // for j in 0..=100000 {
    //     println!("Contains '{}': {}", j, binaryfusefilter.member(j));
    // }
    // for j in 100000..=1000000 {
    //     println!("Contains '{}': {}", j, binaryfusefilter.member(j));
    // }
}

// static size:u64 = ((1.23 * 100f32) + 32.0) as u64;
// static l:u32 = 64 - (size - 1).leading_zeros();
//
// fn get_hashes() -> Vec<(u64, u64, u64)> {
//     let mut rng = rand::thread_rng();
//     let mut hash_functions = Vec::new();
//
//     for _ in 0..=2 {
//         let a1: u64 = rng.gen_range(1..=u64::MAX );
//         let a2: u64 = rng.gen_range(1..=u64::MAX);
//         let b: u64 = rng.gen_range(1..=u64::MAX);
//         hash_functions.push((a1,a2,b));
//     }
//     return hash_functions;
// }
// fn hash0(key: u64, hashes: Vec<(u64,u64,u64)>) -> u32 {
//     let bound = size / 3;
//     let res = hash(key,l, hashes[0].0, hashes[0].1, hashes[0].2) % bound as u32;
//     //println!("'{}','{}'",0, res);
//     return res;
//
// }
//
// fn hash1(key: u64, hashes: Vec<(u64,u64,u64)>) -> u32 {
//     let bound = size / 3;
//
//     let mut res = hash(key,l, hashes[1].0, hashes[1].1, hashes[1].2) % bound as u32;
//     res = (bound + res as u64) as u32;
//     //println!("'{}','{}'",1, res);
//     return res
// }
//
// fn hash2(key: u64, hashes: Vec<(u64,u64,u64)>) -> u32 {
//     let bound = size / 3;
//     let mut res = (hash(key, l, hashes[2].0, hashes[2].1, hashes[2].2)) % bound as u32;
//     res = ((2 * bound) + res as u64) as u32;
//     //println!("'{}','{}'",2, res);
//     return res;
// }