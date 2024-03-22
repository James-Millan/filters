mod bloomfilter;
mod cuckoofilter;
mod countingbloomfilter;
mod xorfilter;
mod bitvector;
mod blockedbloomfilter;
mod utils;
mod registeralignedbloomfilter;
mod threewisebinaryfusefilter32;
mod simdblockedbloomfilter;
mod fourwisebinaryfusefilter;
mod threewisebinaryfusefilter16;
mod threewisebinaryfusefilter8;
mod fpr;
mod XorFilter8;
mod ribbonfilter;
mod registeralignedlarger;
mod tabulationhashing;
mod keygenerator;
#[path = "tabulation/bloomfilter.rs"]
mod btab;
#[path = "tabulation/countingbloomfilter.rs"]
mod cbtab;
#[path = "tabulation/blockedbloomfilter.rs"]
mod bbtab;
#[path = "tabulation/registeralignedbloomfilter.rs"]
mod rabtab;
#[path = "tabulation/cuckoofilter.rs"]
mod ctab;
#[path = "tabulation/XorFilter8.rs"]
mod xtab;
#[path = "tabulation/threewisebinaryfusefilter8.rs"]
mod bftab;
#[path = "tabulation/fpr.rs"]
mod fprtab;

#[path = "fasthash/bloomfilter.rs"]
mod bffast;
#[path = "fasthash/countingbloomfilter.rs"]
mod cbffast;
#[path = "fasthash/blockedbloomfilter.rs"]
mod bbffast;
#[path = "fasthash/registeralignedbloomfilter.rs"]
mod rabbffast;
#[path = "fasthash/fpr.rs"]
mod fprfast;
mod mortonfilter;
mod MortonBlock;
mod quotientfilter;
mod quotientinfo;

extern crate rand;


use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use rand::Rng;
use rand::seq::index::sample;


fn main() {
    let mut keys = vec![];
    for i in 0..100 {
        keys.push(i);
    }
    let size = 1000;

    let mut m = mortonfilter::MortonFilter::new(size, 0.01);
    let mut q = quotientfilter::QuotientFilter::new(size);
    // let mut c = cuckoofilter::CuckooFilter::new(10000,1000,2);
    // let mut nums = vec![];
    // let mut vals = HashMap::new();

    for i in 0..size {
        m.insert(i);
    }

    for i in 0..size {
        println!("contains: {}, {}", i, m.member(i));
    }

    // for j in 1..2 {
    //     let mut q = quotientfilter::QuotientFilter::new(size);
    //     for i in 0..10 {
    //         q.insert(i);
    //         for j in 0..=i {
    //             if !q.member(j) {
    //                 println!("contains: {}, {}", j, q.member(j));
    //             }
    //         }
    //         // println!("contains: {}, {}", i, q.member(i));
    //         // vals.insert(i,m.fingerprint(i));
    //         // c.insert(i);
    //     }
    // }

    // let mut nums = Vec::new();
    // for i in 0..size {
    //     if (q.insert(i)) {
    //         nums.push(i);
    //     }
    // }
    //
    //
    // for i in nums {
    //     println!("contains: {}, {}", i, q.member(i));
    // }


    // m.insert(10);
    // m.insert(10);
    // m.insert(10);
    // m.insert(10);
    // m.insert(10);
    // m.insert(10);

    // println!("contains: {}, {}", 10, m.member(10));

    // let mut fin = vec![];
    // m.delete(10);
    // m.delete(23);
    //
    // for i in 0..=size
    // {
    //     // fin.push(m.fingerprint(i));
    //     // println!("{}", m.fingerprint(i));
    //     // m.delete(i);
    //     if (!m.member(i)){
    //         println!("contains: {}, {}, {:?}", i, m.member(i), vals.get(&i));
    //     }
    //
    // }
    // let t1 = tabulationhashing::TabulationHashing::new();
    // let t2 = tabulationhashing::TabulationHashing::new();
    // let t3 = tabulationhashing::TabulationHashing::new();
    // let mut i = 0;
    // let mut res = vec![];
    // loop {
    //     let hashes = (t1.tabulation_hashing(i),t2.tabulation_hashing(i),t3.tabulation_hashing(i));
    //     if !res.contains(&hashes) {
    //         res.push(hashes);
    //     }
    //     else {
    //         println!("{}", i);
    //         break;
    //     }
    //     println!("{}", i);
    //     i = i + 1;
    // }
    // let mut bf = btab::BloomFilter::new(10000,0.01);
    // let mut cf = ctab::CuckooFilter::new(10000,1000,10);
    // let mut cbf = cbtab::CountingBloomFilter::new(10000,0.01);
    // let mut bbf = bbtab::BlockedBloomFilter::new(10000,512,0.01);
    // let mut rab = rabtab::RegisterAlignedBloomFilter::new(10000,64,0.01);
    // let mut xf = xtab::XorFilter::new(keys.clone());
    // // let mut binf = threewisebinaryfusefilter32::ThreeWiseBinaryFuseFilter32::new(keys.clone());
    // // let mut bff = bftab::ThreeWiseBinaryFuseFilter8::new(keys.clone());
    // let mut bffast = bffast::BloomFilter::new(10000,0.01);
    // let mut cbffast = cbffast::CountingBloomFilter::new(10000,0.01);
    // let mut bbffast = bbffast::BlockedBloomFilter::new(10000,512,0.01);
    // let mut rabbffast = rabbffast::RegisterAlignedBloomFilter::new(10000,64,0.01);
    //
    // for i in 0..=10000 {
    //     bf.insert(i);
    //     cf.insert(i);
    //     cbf.insert(i);
    //     bbf.insert(i);
    //     rab.insert(i);
    //     bffast.insert(i);
    //     cbffast.insert(i);
    //     bbffast.insert(i);
    //     rabbffast.insert(i);
    // }
    //
    //
    // for i in 0..=1000000 {
    //     println!("Contains '{}': {}", i, xf.member(i));
    // }




    // let mut keygen = keygenerator::KeyGenerator::new(100);
    // keygen.write_to_file().expect("TODO: panic message");
    // let mut blank_keygen = keygenerator::KeyGenerator::new_empty();
    // blank_keygen.read_from_file().expect("TODO: panic message");
    // println!("{:?}", keygen.random);
    // println!("{:?}", blank_keygen.random);
    // let sample_size = 1024;
    //
    //
    // fpr::run_fpr_tests(sample_size);
    // // fpr::run_randomised_fpr_tests(sample_size);
    //
    // fprtab::run_fpr_tests(sample_size);
    // // fprtab::run_randomised_fpr_tests(sample_size);
    //
    // fprfast::run_fpr_tests(sample_size);
    // fprfast::run_randomised_fpr_tests(sample_size);

    //,1000,10000,100000,1000000,10000000,100000000
    // let sample_sizes: Vec<u64> = vec![10,100,1000,10000,100000,1000000,100000000];
    // let hasher = tabulationhashing::TabulationHashing::new();
    // for size in sample_sizes {
    //     println!("{}", size);
    //     let mut keys = Vec::new();
    //     for i in 0..=size {
    //         keys.push(i);
    //         // println!("'{}', '{}'",hasher.tabulation_hashing(i), i);
    //     }
    //
    //     fpr::blocked_bloom_filter_fpr(size,0.01,&keys);
    //     fpr::bloom_filter_fpr(size, 0.01, &keys);
    //     fpr::counting_bloom_filter_fpr(size,0.01,&keys);
    //     fpr::cuckoo_filter_fpr(size, 0.01, &keys);
    //     fpr::binary_fuse_filter_8_fpr(&keys);
    //     fpr::xor_filter_fpr(&keys);
    //     fpr::xor_filter_8_fpr(&keys);
    //     fpr::binary_fuse_filter_fpr(&keys);
    //     fpr::blocked_bloom_filter_fpr(size,0.01,&keys);
    //     fpr::register_aligned_bloom_filter_fpr(size,0.01,&keys);
    //     fpr::register_aligned_bloom_filter_larger_fpr(size,0.01,&keys);
    // }
    //


    // let binaryfusefilter = threewisebinaryfusefilter32::ThreeWiseBinaryFuseFilter32::new(keys);
    //
    // for j in 0..=10000 {
    //     println!("Contains '{}': {}", j, binaryfusefilter.member(j));
    // }

    // let ribbon_filter = ribbonfilter::RibbonFilter::new(&keys, 0.1);
    // for j in 0..=10000 {
    //     println!("Contains '{}': {}", j, ribbon_filter.member(j));
    // }


    // let mut ribbon_filter = ribbonfilter::RibbonFilter::new(&keys, 0.1);
    // for j in 0..=10000 {
    //     println!("Contains '{}': {}", j, ribbon_filter.member(j));
    // }
    // //
    // // create simd vectors
    // let x = f32x4(1.0, 2.0, 3.0, 4.0);
    // let y = f32x4(4.0, 3.0, 2.0, 1.0);
    //
    // // simd product
    // let z = x * y;
    //
    // // like any struct, the simd vector can be destructured using `let`
    // let f32x4(a, b, c, d) = z;
    //
    // println!("{:?}", (a, b, c, d));

    // let mut simdBloom = simdblockedbloomfilter::SimdBlockedBloomFilter::new(keys.len() as u64, 64, 0.01);
    // for key in &keys {
    //     simdBloom.insert(*key);
    // }
    //
    // for key in &keys {
    //     println!("{} {}", key, simdBloom.member(*key));
    // }
    // //
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