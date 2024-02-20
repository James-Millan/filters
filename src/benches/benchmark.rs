use std::cell::RefCell;

use criterion::{Criterion};
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use bloomfilter::BloomFilter;
use crate::blockedbloomfilter::BlockedBloomFilter;
use crate::countingbloomfilter::CountingBloomFilter;
use crate::cuckoofilter::CuckooFilter;
use crate::registeralignedbloomfilter::RegisterAlignedBloomFilter;

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

static SAMPLE_SIZE: u64 = 100000000;

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

    //let mut bloom_filter = BloomFilter::new(100000, 6).clone();
    let mut i = 0;
    c.bench_function("bench_bloom_filter_insert", |b| {
        b.iter(|| {
            bloom_filter.borrow_mut().insert(i);
            i += 1;
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
    let mut i = 0;
    c.bench_function("bench_bloom_filter_member", |b| {
        b.iter(|| {
            bloom_filter.borrow_mut().member(i);
            i += 1;
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
    let _sample_size = 100000000;
    let cuckoo_filter = RefCell::new(CuckooFilter::new(SAMPLE_SIZE as usize, 10000, 8));

    //let mut bloom_filter = BloomFilter::new(100000, 6).clone();
    let mut i = 0;
    c.bench_function("bench_cuckoofilter_insert", |b| {
        b.iter(|| {
            cuckoo_filter.borrow_mut().insert(i);
            i += 1;
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
    let mut i = 0;
    c.bench_function("bench_cuckoofilter_member", |b| {
        b.iter(|| {
            cuckoo_filter.borrow_mut().member(i);
            i += 1;
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
    let sample_size = 1000000000;
    let counting_bloom_filter = RefCell::new(CountingBloomFilter::new(SAMPLE_SIZE, 0.01));
    //let mut bloom_filter = BloomFilter::new(100000, 6).clone();
    let mut i = 0;
    c.bench_function("bench_counting_bloom_filter_insert", |b| {
        b.iter(|| {
            counting_bloom_filter.borrow_mut().insert(i);
            i += 1;
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
    let mut i = 0;
    c.bench_function("bench_counting_bloom_filter_member", |b| {
        b.iter(|| {
            counting_bloom_filter.borrow_mut().member(i);
            i += 1;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
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
    let mut i = 0;
    c.bench_function("bench_blocked_bloom_filter_insert", |b| {
        b.iter(|| {
            blocked_bloom_filter.borrow_mut().insert(i);
            i += 1;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}

fn bench_blocked_bloom_filter_query(c: &mut Criterion) {
    let _sample_size = 1000000000;
    let blocked_bloom_filter = RefCell::new(BlockedBloomFilter::new(SAMPLE_SIZE,
                                                                        64, 0.01));
    //let mut bloom_filter = BloomFilter::new(100000, 6).clone();

    for j in 0..100000 {
        blocked_bloom_filter.borrow_mut().insert(j);
    }

    let mut i = 0;
    c.bench_function("bench_blocked_bloom_filter_query", |b| {
        b.iter(|| {
            blocked_bloom_filter.borrow_mut().member(i);
            i += 1;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
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
    let mut i = 0;
    c.bench_function("bench_register_aligned_bloom_filter_insert", |b| {
        b.iter(|| {
            register_aligned_bloom_filter.borrow_mut().insert(i);
            i += 1;
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
    let mut i = 0;
    c.bench_function("bench_register_aligned_filter_member", |b| {
        b.iter(|| {
            register_aligned_filter.borrow_mut().member(i);
            i += 1;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
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
//     let mut i = 0;
//     c.bench_function("bench_simd_blocked_bloom_filter_insert", |b| {
//         b.iter(|| {
//             simd_blocked_bloom_filter.borrow_mut().insert(i);
//             i += 1;
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

    //let mut bloom_filter = BloomFilter::new(100000, 6).clone();
    let mut i = 0;
    c.bench_function("bench_xor_filter_member", |b| {
        b.iter(|| {
            xor_filter.borrow_mut().member(i);
            i += 1;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
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
            //stop it being optimized by the compiler
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
    let mut i = 0;
    c.bench_function("bench_binary_fuse_filter_member", |b| {
        b.iter(|| {
            binary_fuse_filter.borrow_mut().member(i);
            i += 1;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
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
            //stop it being optimized by the compiler
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
    let mut i = 0;
    c.bench_function("bench_four_wise_binary_fuse_filter_member", |b| {
        b.iter(|| {
            binary_fuse_filter.borrow_mut().member(i);
            i += 1;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
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
    group.sample_size(10 as usize);
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
    setup, bench_bloom_filter_insert, bench_bloom_filter_member,
    // bench_cuckoo_filter_insert, 
    bench_cuckoo_filter_member, bench_counting_bloom_filter_insert, 
    bench_counting_bloom_filter_member,bench_blocked_bloom_filter_insert,
    bench_blocked_bloom_filter_query, 
    bench_register_aligned_bloom_filter_insert, bench_register_aligned_filter_member  , setup2, bench_bloom_filter_create,  bench_cuckoo_filter_create,
    bench_binary_fuse_filter_create,bench_counting_bloom_filter_create,bench_blocked_bloom_filter_create,bench_register_aligned_bloom_filter_create,
    bench_xor_filter_create, setup3,
    // bench_four_wise_binary_fuse_filter_create,
    bench_binary_fuse_filter_query,bench_xor_filter_query
    // ,bench_four_wise_binary_fuse_filter_query
);
criterion_main!(benches);
