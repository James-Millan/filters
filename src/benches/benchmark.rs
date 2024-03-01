use std::cell::RefCell;
use std::collections::HashSet;

use criterion::{Criterion};
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use rand::prelude::SliceRandom;
use rand::Rng;
use bloomfilter::BloomFilter;
use crate::blockedbloomfilter::BlockedBloomFilter;
use crate::countingbloomfilter::CountingBloomFilter;
use crate::cuckoofilter::CuckooFilter;
use crate::registeralignedbloomfilter::RegisterAlignedBloomFilter;

#[path = "../keygenerator.rs"]
mod keygenerator;
#[path = "../bloomfilter.rs"]
mod bloomfilter;

#[path = "../blockedbloomfilter.rs"]
mod blockedbloomfilter;

#[path = "../cuckoofilter.rs"]
mod cuckoofilter;

#[path = "../countingbloomfilter.rs"]
mod countingbloomfilter;

#[path = "../registeralignedbloomfilter.rs"]
mod registeralignedbloomfilter;
#[path = "../simdblockedbloomfilter.rs"]
mod simdblockedbloomfilter;
#[path = "../xorfilter.rs"]
mod xorfilter;

#[path = "../threewisebinaryfusefilter32.rs"]
mod binaryfusefiter3;
#[path = "../fourwisebinaryfusefilter.rs"]
mod binaryfusefiter4;

static SAMPLE_SIZE: u64 = 10000;
// let DISJOINT_KEYS:(Vec<u64>, Vec<u64>) = ((0..SAMPLE_SIZE).collect(),(SAMPLE_SIZE..2*SAMPLE_SIZE).collect());
// let MIXED_KEYS:(Vec<u64>, Vec<u64>) = generate_mixed_keys(SAMPLE_SIZE);
// let UNIFORM_KEYS: Vec<u64> = (0..SAMPLE_SIZE).collect();
// let RANDOM_KEYS: (Vec<u64>, Vec<u64>) = generate_random_keys(SAMPLE_SIZE);

fn bench_bloom_filter_create(c: &mut Criterion) {
    c.bench_function("bench_bloom_filter_create", |b| {
        b.iter(|| {
            let mut bloom_filter = BloomFilter::new(SAMPLE_SIZE, 0.01);
            for i in 0..=SAMPLE_SIZE {
                bloom_filter.insert(i);
            }
            //stop it being optimized by the compiler
            black_box(bloom_filter);
        });
    });
}

fn bench_bloom_filter_insert(c: &mut Criterion) {
    let bloom_filter = RefCell::new(BloomFilter::new(SAMPLE_SIZE, 0.01));
    let mut key_gen = keygenerator::KeyGenerator::new(SAMPLE_SIZE);
    key_gen.write_to_file().expect("error writing");

    //let mut bloom_filter = BloomFilter::new(100000, 6).clone();
    let mut i: usize = 0;
    c.bench_function("bench_bloom_filter_insert", |b| {
        b.iter(|| {
            bloom_filter.borrow_mut().insert(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize);
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}

fn bench_bloom_filter_member(c: &mut Criterion) {
    let bloom_filter = RefCell::new(BloomFilter::new(SAMPLE_SIZE, 0.01));
    for j in 0..SAMPLE_SIZE {
        bloom_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_bloom_filter_member", |b| {
        b.iter(|| {
            bloom_filter.borrow_mut().member(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}
fn bench_bloom_filter_random_member(c: &mut Criterion) {
    let bloom_filter = RefCell::new(BloomFilter::new(SAMPLE_SIZE, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let random_keys = keys.random;

    for j in random_keys.0 {
        bloom_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_bloom_filter_random_member", |b| {
        b.iter(|| {
            bloom_filter.borrow_mut().member(random_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}
fn bench_bloom_filter_disjoint_member(c: &mut Criterion) {
    let bloom_filter = RefCell::new(BloomFilter::new(SAMPLE_SIZE, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("error opening file");
    let disjoint_keys = keys.disjoint;
    for j in disjoint_keys.0 {
        bloom_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_bloom_filter_disjoint_member", |b| {
        b.iter(|| {
            bloom_filter.borrow_mut().member(disjoint_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}

fn bench_bloom_filter_mixed_member(c: &mut Criterion) {
    let bloom_filter = RefCell::new(BloomFilter::new(SAMPLE_SIZE, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let mixed_keys = keys.mixed;
    for j in mixed_keys.0 {
        bloom_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_bloom_filter_mixed_member", |b| {
        b.iter(|| {
            bloom_filter.borrow_mut().member(mixed_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}
fn bench_cuckoo_filter_create(c: &mut Criterion) {
    c.bench_function("bench_cuckoo_filter_create", |b| {
        b.iter(|| {
            let mut cuckoo_filter = CuckooFilter::new(SAMPLE_SIZE as usize, 1000, 8);
            for i in 0..=SAMPLE_SIZE {
                cuckoo_filter.insert(i);
            }
            //stop it being optimized by the compiler
            black_box(cuckoo_filter);
        });
    });
}

fn bench_cuckoo_filter_insert(c: &mut Criterion) {
    let cuckoo_filter = RefCell::new(CuckooFilter::new(SAMPLE_SIZE as usize, SAMPLE_SIZE as usize, 8));

    //let mut bloom_filter = BloomFilter::new(100000, 6).clone();
    let mut i: usize = 0;
    c.bench_function("bench_cuckoofilter_insert", |b| {
        b.iter(|| {
            cuckoo_filter.borrow_mut().insert(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}

fn bench_cuckoo_filter_member(c: &mut Criterion) {
    let cuckoo_filter = RefCell::new(CuckooFilter::new(SAMPLE_SIZE as usize, 10000, 8));

    for j in 0..SAMPLE_SIZE {
        cuckoo_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_cuckoofilter_member", |b| {
        b.iter(|| {
            cuckoo_filter.borrow_mut().member(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}

fn bench_cuckoo_filter_random_member(c: &mut Criterion) {
    let cuckoo_filter = RefCell::new(CuckooFilter::new(SAMPLE_SIZE as usize, 10000, 8));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let random_keys = keys.random;
    for j in random_keys.0 {
        cuckoo_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_cuckoo_filter_random_member", |b| {
        b.iter(|| {
            cuckoo_filter.borrow_mut().member(random_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}
fn bench_cuckoo_filter_disjoint_member(c: &mut Criterion) {
    let cuckoo_filter = RefCell::new(CuckooFilter::new(SAMPLE_SIZE as usize, 10000, 8));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let disjoint_keys = keys.disjoint;
    for j in disjoint_keys.0 {
        cuckoo_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_cuckoo_filter_disjoint_member", |b| {
        b.iter(|| {
            cuckoo_filter.borrow_mut().member(disjoint_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}
fn bench_cuckoo_filter_mixed_member(c: &mut Criterion) {
    let cuckoo_filter = RefCell::new(CuckooFilter::new(SAMPLE_SIZE as usize, 10000, 8));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let mixed_keys = keys.mixed;
    for j in mixed_keys.0 {
        cuckoo_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_cuckoo_filter_mixed_member", |b| {
        b.iter(|| {
            cuckoo_filter.borrow_mut().member(mixed_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}


fn bench_counting_bloom_filter_create(c: &mut Criterion) {
    c.bench_function("bench_counting_bloom_filter_create", |b| {
        b.iter(|| {
            let mut counting_bloom__filter = CountingBloomFilter::new(SAMPLE_SIZE, 0.01);
            for i in 0..=SAMPLE_SIZE {
                counting_bloom__filter.insert(i);
            }
            //stop it being optimized by the compiler
            black_box(counting_bloom__filter);
        });
    });
}

fn bench_counting_bloom_filter_insert(c: &mut Criterion) {
    let counting_bloom_filter = RefCell::new(CountingBloomFilter::new(SAMPLE_SIZE, 0.01));
    //let mut bloom_filter = BloomFilter::new(100000, 6).clone();
    let mut i: usize = 0;
    c.bench_function("bench_counting_bloom_filter_insert", |b| {
        b.iter(|| {
            counting_bloom_filter.borrow_mut().insert(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
            //stop it being optimized by the compiler
        });
        black_box(counting_bloom_filter.borrow_mut());
    });
}

fn bench_counting_bloom_filter_member(c: &mut Criterion) {
    let counting_bloom_filter = RefCell::new(CountingBloomFilter::new(SAMPLE_SIZE, 0.01));

    for j in 0..SAMPLE_SIZE {
        counting_bloom_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_counting_bloom_filter_member", |b| {
        b.iter(|| {
            counting_bloom_filter.borrow_mut().member(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_counting_bloom_filter_random_member(c: &mut Criterion) {
    let counting_bloom_filter = RefCell::new(CountingBloomFilter::new(SAMPLE_SIZE, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let random_keys = keys.random;
    for j in random_keys.0 {
        counting_bloom_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_counting_bloom_filter_random_member", |b| {
        b.iter(|| {
            counting_bloom_filter.borrow_mut().member(random_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_counting_bloom_filter_disjoint_member(c: &mut Criterion) {
    let counting_bloom_filter = RefCell::new(CountingBloomFilter::new(SAMPLE_SIZE, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let disjoint_keys = keys.disjoint;
    for j in disjoint_keys.0 {
        counting_bloom_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_counting_bloom_filter_disjoint_member", |b| {
        b.iter(|| {
            counting_bloom_filter.borrow_mut().member(disjoint_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}

fn bench_counting_bloom_filter_mixed_member(c: &mut Criterion) {
    let counting_bloom_filter = RefCell::new(CountingBloomFilter::new(SAMPLE_SIZE, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let mixed_keys = keys.mixed;
    for j in mixed_keys.0 {
        counting_bloom_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_counting_bloom_filter_mixed_member", |b| {
        b.iter(|| {
            counting_bloom_filter.borrow_mut().member(mixed_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_blocked_bloom_filter_create(c: &mut Criterion) {
    c.bench_function("bench_blocked_bloom_filter_create", |b| {
        b.iter(|| {
            let mut blocked_bloom_filter = BlockedBloomFilter::new(SAMPLE_SIZE, 64, 0.01);
            for i in 0..=SAMPLE_SIZE {
                blocked_bloom_filter.insert(i);
            }
            //stop it being optimized by the compiler
            black_box(blocked_bloom_filter);
        });
    });
}
fn bench_blocked_bloom_filter_insert(c: &mut Criterion) {
    let _sample_size = 1000000000;
    let blocked_bloom_filter = RefCell::new(BlockedBloomFilter::new(SAMPLE_SIZE,
                                                                        64, 0.01));
    //let mut bloom_filter = BloomFilter::new(100000, 6).clone();
    let mut i: usize = 0;
    c.bench_function("bench_blocked_bloom_filter_insert", |b| {
        b.iter(|| {
            blocked_bloom_filter.borrow_mut().insert(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}
fn bench_blocked_bloom_filter_query(c: &mut Criterion) {
    let blocked_bloom_filter = RefCell::new(BlockedBloomFilter::new(SAMPLE_SIZE,
                                                                        64, 0.01));
    for j in 0..SAMPLE_SIZE {
        blocked_bloom_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_blocked_bloom_filter_query", |b| {
        b.iter(|| {
            blocked_bloom_filter.borrow_mut().member(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}

fn bench_blocked_bloom_filter_random_query(c: &mut Criterion) {
    let blocked_bloom_filter = RefCell::new(BlockedBloomFilter::new(SAMPLE_SIZE,
                                                                    64, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let random_keys = keys.random;
    for j in random_keys.0 {
        blocked_bloom_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_blocked_bloom_filter_random_query", |b| {
        b.iter(|| {
            blocked_bloom_filter.borrow_mut().member(random_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}

fn bench_blocked_bloom_filter_disjoint_query(c: &mut Criterion) {
    let blocked_bloom_filter = RefCell::new(BlockedBloomFilter::new(SAMPLE_SIZE,
                                                                    64, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let disjoint_keys = keys.disjoint;
    for j in disjoint_keys.0 {
        blocked_bloom_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_blocked_bloom_filter_disjoint_query", |b| {
        b.iter(|| {
            blocked_bloom_filter.borrow_mut().member(disjoint_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}

fn bench_blocked_bloom_filter_mixed_query(c: &mut Criterion) {
    let blocked_bloom_filter = RefCell::new(BlockedBloomFilter::new(SAMPLE_SIZE,
                                                                    64, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let mixed_keys = keys.mixed;
    for j in mixed_keys.0 {
        blocked_bloom_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_blocked_bloom_filter_mixed_query", |b| {
        b.iter(|| {
            blocked_bloom_filter.borrow_mut().member(mixed_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_register_aligned_bloom_filter_create(c: &mut Criterion) {
    c.bench_function("bench_register_aligned_bloom_filter_create", |b| {
        b.iter(|| {
            let mut register_aligned_bloom_filter = registeralignedbloomfilter::
            RegisterAlignedBloomFilter::new(SAMPLE_SIZE, 64, 0.01);
            for i in 0..=SAMPLE_SIZE {
                register_aligned_bloom_filter.insert(i);
            }
            //stop it being optimized by the compiler
            black_box(register_aligned_bloom_filter);
        });
    });
}
fn bench_register_aligned_bloom_filter_insert(c: &mut Criterion) {
    let register_aligned_bloom_filter = RefCell::new(registeralignedbloomfilter
    ::RegisterAlignedBloomFilter::new(SAMPLE_SIZE, 64, 0.01));

    //let mut bloom_filter = BloomFilter::new(100000, 6).clone();
    let mut i: usize = 0;
    c.bench_function("bench_register_aligned_bloom_filter_insert", |b| {
        b.iter(|| {
            register_aligned_bloom_filter.borrow_mut().insert(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}
fn bench_register_aligned_filter_member(c: &mut Criterion) {
    let register_aligned_filter = RefCell::new(RegisterAlignedBloomFilter::new(SAMPLE_SIZE,
                                                                     64, 0.01));
    for j in 0..SAMPLE_SIZE {
        register_aligned_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_register_aligned_filter_member", |b| {
        b.iter(|| {
            register_aligned_filter.borrow_mut().member(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}

fn bench_register_aligned_filter_random_member(c: &mut Criterion) {
    let register_aligned_filter = RefCell::new(RegisterAlignedBloomFilter::new(SAMPLE_SIZE,
                                                                               64, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let random_keys = keys.random;
    for j in random_keys.0 {
        register_aligned_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_register_aligned_filter_random_member", |b| {
        b.iter(|| {
            register_aligned_filter.borrow_mut().member(random_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_register_aligned_filter_disjoint_member(c: &mut Criterion) {
    let register_aligned_filter = RefCell::new(RegisterAlignedBloomFilter::new(SAMPLE_SIZE,
                                                                               64, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let disjoint_keys = keys.disjoint;
    for j in disjoint_keys.0 {
        register_aligned_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_register_aligned_filter_disjoint_member", |b| {
        b.iter(|| {
            register_aligned_filter.borrow_mut().member(disjoint_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_register_aligned_filter_mixed_member(c: &mut Criterion) {
    let register_aligned_filter = RefCell::new(RegisterAlignedBloomFilter::new(SAMPLE_SIZE,
                                                                               64, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let mixed_keys = keys.mixed;
    for j in mixed_keys.0 {
        register_aligned_filter.borrow_mut().insert(j);
    }
    let mut i: usize = 0;
    c.bench_function("bench_register_aligned_filter_mixed_member", |b| {
        b.iter(|| {
            register_aligned_filter.borrow_mut().member(mixed_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}

// fn bench_simd_blocked_bloom_filter_create(c: &mut Criterion) {
//     c.bench_function("bench_simd_blocked_bloom_filter_create", |b| {
//         b.iter(|| {
//             let simd_blocked_bloom_filter = simdblockedbloomfilter::SimdBlockedBloomFilter::new(
//                 100000,64,0.01);
//             //stop it being optimized by the compiler
//             black_box(simd_blocked_bloom_filter);
//         });
//     });
// }

// fn bench_simd_blocked_bloom_filter_insert(c: &mut Criterion) {
//     let _sample_size = 1000000;
//     let simd_blocked_bloom_filter = RefCell::new(simdblockedbloomfilter::SimdBlockedBloomFilter::new(
//         100000,64,0.01));
//
//     //let mut bloom_filter = BloomFilter::new(100000, 6).clone();
//     let mut i: usize = 0;
//     c.bench_function("bench_simd_blocked_bloom_filter_insert", |b| {
//         b.iter(|| {
//             simd_blocked_bloom_filter.borrow_mut().insert(i);
//             i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
//             //stop it being optimized by the compiler
//             //black_box(bloom_filter);
//         });
//     });
// }

fn bench_xor_filter_create(c: &mut Criterion) {
    c.bench_function("bench_xor_filter_create", |b| {
        let mut keys = Vec::new();
        for i in 0..SAMPLE_SIZE {
            keys.push(i);
        }
        b.iter(|| {
            let xor_filter = xorfilter::XorFilter::new(keys.clone());
            //stop it being optimized by the compiler
            black_box(xor_filter);
        });
    });
}

fn bench_xor_filter_query(c: &mut Criterion) {
    let mut keys = Vec::new();
    for i in 0..SAMPLE_SIZE {
        keys.push(i);
    }
    let xor_filter = RefCell::new(xorfilter::XorFilter::new(keys.clone()));
    let mut i: usize = 0;
    c.bench_function("bench_xor_filter_query", |b| {
        b.iter(|| {
            xor_filter.borrow_mut().member(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}

fn bench_xor_filter_random_query(c: &mut Criterion) {
    let mut keys = Vec::new();
    let mut keys_gen = keygenerator::KeyGenerator::new_empty();
    keys_gen.read_from_file().expect("");
    let random_keys = keys_gen.random;
    for i in random_keys.0 {
        keys.push(i);
    }
    let xor_filter = RefCell::new(xorfilter::XorFilter::new(keys.clone()));
    let mut i: usize = 0;
    c.bench_function("bench_xor_filter_random_query", |b| {
        b.iter(|| {
            xor_filter.borrow_mut().member(random_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_xor_filter_disjoint_query(c: &mut Criterion) {
    let mut keys = Vec::new();
    let mut keys_gen = keygenerator::KeyGenerator::new_empty();
    keys_gen.read_from_file().expect("");
    let disjoint_keys = keys_gen.disjoint;
    for i in disjoint_keys.0 {
        keys.push(i);
    }
    let xor_filter = RefCell::new(xorfilter::XorFilter::new(keys.clone()));
    let mut i: usize = 0;
    c.bench_function("bench_xor_filter_disjoint_query", |b| {
        b.iter(|| {
            xor_filter.borrow_mut().member(disjoint_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_xor_filter_mixed_query(c: &mut Criterion) {
    let mut keys = Vec::new();
    let mut keys_gen = keygenerator::KeyGenerator::new_empty();
    keys_gen.read_from_file().expect("");
    let mixed_keys = keys_gen.mixed;
    for i in mixed_keys.0 {
        keys.push(i);
    }
    let xor_filter = RefCell::new(xorfilter::XorFilter::new(keys.clone()));
    let mut i: usize = 0;
    c.bench_function("bench_xor_filter_mixed_query", |b| {
        b.iter(|| {
            xor_filter.borrow_mut().member(mixed_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_binary_fuse_filter_create(c: &mut Criterion) {
    c.bench_function("bench_binary_fuse_filter_create", |b| {
        let mut keys = Vec::new();
        for i in 0..SAMPLE_SIZE {
            keys.push(i);
        }
        b.iter(|| {
            let binary_fuse_filter = binaryfusefiter3::ThreeWiseBinaryFuseFilter32::new(keys.clone());
            black_box(binary_fuse_filter);
        });
    });
}
fn bench_binary_fuse_filter_query(c: &mut Criterion) {
    let mut keys = Vec::new();
    for i in 0..SAMPLE_SIZE {
        keys.push(i);
    }
    let binary_fuse_filter = RefCell::new(binaryfusefiter3::ThreeWiseBinaryFuseFilter32::new(keys.clone()));
    let mut i: usize = 0;
    c.bench_function("bench_binary_fuse_filter_member", |b| {
        b.iter(|| {
            binary_fuse_filter.borrow_mut().member(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_binary_fuse_filter_random_query(c: &mut Criterion) {
    let mut keys = Vec::new();
    let mut keys_gen = keygenerator::KeyGenerator::new_empty();
    keys_gen.read_from_file().expect("");
    let random_keys = keys_gen.random;
    for i in random_keys.0 {
        keys.push(i);
    }
    let binary_fuse_filter = RefCell::new(binaryfusefiter3::ThreeWiseBinaryFuseFilter32::new(keys.clone()));
    let mut i: usize = 0;
    c.bench_function("bench_binary_fuse_filter_random_query", |b| {
        b.iter(|| {
            binary_fuse_filter.borrow_mut().member(random_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_binary_fuse_filter_disjoint_query(c: &mut Criterion) {
    let mut keys = Vec::new();
    let mut keys_gen = keygenerator::KeyGenerator::new_empty();
    keys_gen.read_from_file().expect("");
    let disjoint_keys = keys_gen.disjoint;
    for i in disjoint_keys.0 {
        keys.push(i);
    }
    let binary_fuse_filter = RefCell::new(binaryfusefiter3::ThreeWiseBinaryFuseFilter32::new(keys.clone()));
    let mut i: usize = 0;
    c.bench_function("bench_binary_fuse_filter_disjoint_query", |b| {
        b.iter(|| {
            binary_fuse_filter.borrow_mut().member(disjoint_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_binary_fuse_filter_mixed_query(c: &mut Criterion) {
    let mut keys = Vec::new();
    let mut keys_gen = keygenerator::KeyGenerator::new_empty();
    keys_gen.read_from_file().expect("");
    let mixed_keys = keys_gen.mixed;
    for i in mixed_keys.0 {
        keys.push(i);
    }
    let binary_fuse_filter = RefCell::new(binaryfusefiter3::ThreeWiseBinaryFuseFilter32::new(keys.clone()));
    let mut i: usize = 0;
    c.bench_function("bench_binary_fuse_filter_mixed_query", |b| {
        b.iter(|| {
            binary_fuse_filter.borrow_mut().member(mixed_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_four_wise_binary_fuse_filter_create(c: &mut Criterion) {
    c.bench_function("bench_four_wise_binary_fuse_filter_create", |b| {
        let mut keys = Vec::new();
        for i in 0..SAMPLE_SIZE {
            keys.push(i);
        }
        b.iter(|| {
            let binary_fuse_filter = binaryfusefiter4::FourWiseBinaryFuseFilter::new(&keys);
            black_box(binary_fuse_filter);
        });
    });
}

fn bench_four_wise_binary_fuse_filter_query(c: &mut Criterion) {
    let mut keys = Vec::new();
    for i in 0..SAMPLE_SIZE {
        keys.push(i);
    }
    let binary_fuse_filter = RefCell::new(binaryfusefiter4::FourWiseBinaryFuseFilter::new(&keys));
    let mut i: usize = 0;
    c.bench_function("bench_four_wise_binary_fuse_filter_member", |b| {
        b.iter(|| {
            binary_fuse_filter.borrow_mut().member(i as u64);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_four_wise_binary_fuse_filter_random_query(c: &mut Criterion) {
    let mut keys = Vec::new();
    let mut keys_gen = keygenerator::KeyGenerator::new_empty();
    keys_gen.read_from_file().expect("");
    let random_keys = keys_gen.random;
    for i in random_keys.0 {
        keys.push(i);
    }
    let binary_fuse_filter = RefCell::new(binaryfusefiter4::FourWiseBinaryFuseFilter::new(&keys));
    let mut i: usize = 0;
    c.bench_function("bench_four_wise_binary_fuse_filter_random_member", |b| {
        b.iter(|| {
            binary_fuse_filter.borrow_mut().member(random_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}
fn bench_four_wise_binary_fuse_filter_disjoint_query(c: &mut Criterion) {
    let mut keys = Vec::new();
    let mut keys_gen = keygenerator::KeyGenerator::new_empty();
    keys_gen.read_from_file().expect("");
    let disjoint_keys = keys_gen.disjoint;
    for i in disjoint_keys.0 {
        keys.push(i);
    }
    let binary_fuse_filter = RefCell::new(binaryfusefiter4::FourWiseBinaryFuseFilter::new(&keys));
    let mut i: usize = 0;
    c.bench_function("bench_four_wise_binary_fuse_filter_disjoint_member", |b| {
        b.iter(|| {
            binary_fuse_filter.borrow_mut().member(disjoint_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}

fn bench_four_wise_binary_fuse_filter_mixed_query(c: &mut Criterion) {
    let mut keys = Vec::new();
    let mut keys_gen = keygenerator::KeyGenerator::new_empty();
    keys_gen.read_from_file().expect("");
    let mixed_keys = keys_gen.mixed;
    for i in mixed_keys.0 {
        keys.push(i);
    }
    let binary_fuse_filter = RefCell::new(binaryfusefiter4::FourWiseBinaryFuseFilter::new(&keys));
    let mut i: usize = 0;
    c.bench_function("bench_four_wise_binary_fuse_filter_mixed_member", |b| {
        b.iter(|| {
            binary_fuse_filter.borrow_mut().member(mixed_keys.1[i]);
            i = ((i + 1usize) % SAMPLE_SIZE as usize) as usize;
        });
    });
}

fn setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("benches");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.sample_size(SAMPLE_SIZE as usize);
    // group.finish();
}

fn setup2(c: &mut Criterion) {
    let mut group = c.benchmark_group("benches");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.sample_size(10usize);
    // group.finish();
}

fn setup3(c: &mut Criterion) {
    let mut group = c.benchmark_group("benches");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.sample_size(SAMPLE_SIZE as usize);
    // group.finish();
}

//criterion_group!(current_benches,setup);
criterion_group!(benches,
    setup, bench_bloom_filter_insert, bench_bloom_filter_member, bench_bloom_filter_random_member, bench_bloom_filter_disjoint_member,
    bench_bloom_filter_mixed_member, bench_cuckoo_filter_member, bench_cuckoo_filter_random_member,
    bench_cuckoo_filter_disjoint_member, bench_cuckoo_filter_mixed_member,
    bench_counting_bloom_filter_insert, bench_counting_bloom_filter_member, bench_counting_bloom_filter_random_member,
    bench_counting_bloom_filter_disjoint_member, bench_counting_bloom_filter_mixed_member, bench_blocked_bloom_filter_insert
    ,bench_blocked_bloom_filter_query,bench_blocked_bloom_filter_random_query,
    bench_blocked_bloom_filter_disjoint_query,bench_blocked_bloom_filter_mixed_query,
    bench_register_aligned_bloom_filter_insert, bench_register_aligned_filter_member, bench_register_aligned_filter_random_member,
    bench_register_aligned_filter_disjoint_member, bench_register_aligned_filter_mixed_member,
    setup2, bench_bloom_filter_create,  bench_cuckoo_filter_create,
    bench_binary_fuse_filter_create,bench_counting_bloom_filter_create,bench_blocked_bloom_filter_create,bench_register_aligned_bloom_filter_create,
    bench_xor_filter_create, setup3,
    // bench_four_wise_binary_fuse_filter_create,
    bench_binary_fuse_filter_query,bench_binary_fuse_filter_random_query, bench_binary_fuse_filter_mixed_query,
    bench_binary_fuse_filter_disjoint_query,bench_xor_filter_query, bench_xor_filter_random_query, bench_xor_filter_mixed_query,
    bench_xor_filter_disjoint_query
    // ,bench_four_wise_binary_fuse_filter_query
);
criterion_main!(benches);
