use std::cell::RefCell;
use std::collections::HashSet;
use std::time::Instant;

use criterion::{Criterion};
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use rand::prelude::SliceRandom;
use rand::Rng;



#[path = "../fasthash/bloomfilter.rs"]
mod bloomfilter;

#[path = "../fasthash/blockedbloomfilter.rs"]
mod blockedbloomfilter;

#[path = "../fasthash/countingbloomfilter.rs"]
mod countingbloomfilter;

#[path = "../fasthash/registeralignedbloomfilter.rs"]
mod registeralignedbloomfilter;

#[path = "../keygenerator.rs"]
mod keygenerator;

static  SAMPLE_SIZE: u64 = keygenerator::SAMPLE_SIZE;



fn bench_bloom_filter_fasthash_member(c: &mut Criterion) {
    // setup
    let bloom_filter = RefCell::new(bloomfilter::BloomFilter::new(SAMPLE_SIZE, 0.01));
    let mut keys = keygenerator::KeyGenerator::new(SAMPLE_SIZE);
    keys.write_to_file().expect("");
    let disjoint_keys = keys.disjoint;

    for j in disjoint_keys.0.clone() {
        bloom_filter.borrow_mut().insert(j);
    }

    // custom benchmarking function.
    c.bench_function("bench_bloom_filter_fasthash_member", |b| {
        b.iter_custom(|iters| {
            let mut num_runs = (iters as f64 / SAMPLE_SIZE as f64).ceil() as u64;
            if num_runs <= 1 {
                //  just run the for loop form 0 - iters in here.
                let start = Instant::now();
                for i in 0..(iters as usize) {
                    black_box(bloom_filter.borrow().member(disjoint_keys.0[i]));
                    // xor_filter.borrow().member(random_keys.1[i]);
                }
                return start.elapsed();
            }
            let start = Instant::now();
            for _ in 0..num_runs {
                for i in 0..(SAMPLE_SIZE as usize) {
                    // check 1st pair, i.e the same that we inserted.
                    black_box(bloom_filter.borrow().member(disjoint_keys.0[i]));
                }
            }
            return start.elapsed();
        });
    });
}
fn bench_counting_bloom_filter_fasthash_member(c: &mut Criterion) {
    // setup
    let counting_bloom_filter = RefCell::new(countingbloomfilter::CountingBloomFilter::new(SAMPLE_SIZE, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let disjoint_keys = keys.disjoint;

    for j in disjoint_keys.0.clone() {
        counting_bloom_filter.borrow_mut().insert(j);
    }

    // custom benchmarking function.
    c.bench_function("bench_counting_bloom_filter_fasthash_member", |b| {
        b.iter_custom(|iters| {
            let mut num_runs = (iters as f64 / SAMPLE_SIZE as f64).ceil() as u64;
            if num_runs <= 1 {
                //  just run the for loop form 0 - iters in here.
                let start = Instant::now();
                for i in 0..(iters as usize) {
                    black_box(counting_bloom_filter.borrow().member(disjoint_keys.0[i]));
                    // xor_filter.borrow().member(random_keys.1[i]);
                }
                return start.elapsed();
            }
            let start = Instant::now();
            for _ in 0..num_runs {
                for i in 0..(SAMPLE_SIZE as usize) {
                    // check 1st pair, i.e the same that we inserted.
                    black_box(counting_bloom_filter.borrow().member(disjoint_keys.0[i]));
                }
            }
            return start.elapsed();
        });
    });
}
fn bench_blocked_bloom_filter_fasthash_member(c: &mut Criterion) {
    // setup
    let blocked_bloom_filter = RefCell::new(blockedbloomfilter::BlockedBloomFilter::new(SAMPLE_SIZE, 512, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let disjoint_keys = keys.disjoint;

    for j in disjoint_keys.0.clone() {
        blocked_bloom_filter.borrow_mut().insert(j);
    }

    // custom benchmarking function.
    c.bench_function("bench_blocked_bloom_filter_fasthash_member", |b| {
        b.iter_custom(|iters| {
            let mut num_runs = (iters as f64 / SAMPLE_SIZE as f64).ceil() as u64;
            if num_runs <= 1 {
                //  just run the for loop form 0 - iters in here.
                let start = Instant::now();
                for i in 0..(iters as usize) {
                    black_box(blocked_bloom_filter.borrow().member(disjoint_keys.0[i]));
                    // xor_filter.borrow().member(random_keys.1[i]);
                }
                return start.elapsed();
            }
            let start = Instant::now();
            for _ in 0..num_runs {
                for i in 0..(SAMPLE_SIZE as usize) {
                    // check 1st pair, i.e the same that we inserted.
                    black_box(blocked_bloom_filter.borrow().member(disjoint_keys.0[i]));
                }
            }
            return start.elapsed();
        });
    });
}
fn bench_register_aligned_bloom_filter_fasthash_member(c: &mut Criterion) {
    // setup
    let register_aligned_bloom_filter = RefCell::new(
        registeralignedbloomfilter::RegisterAlignedBloomFilter::new(SAMPLE_SIZE, 64, 0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let disjoint_keys = keys.disjoint;

    for j in disjoint_keys.0.clone() {
        register_aligned_bloom_filter.borrow_mut().insert(j);
    }

    // custom benchmarking function.
    c.bench_function("bench_register_aligned_bloom_filter_fasthash_member", |b| {
        b.iter_custom(|iters| {
            let mut num_runs = (iters as f64 / SAMPLE_SIZE as f64).ceil() as u64;
            if num_runs <= 1 {
                //  just run the for loop form 0 - iters in here.
                let start = Instant::now();
                for i in 0..(iters as usize) {
                    black_box(register_aligned_bloom_filter.borrow().member(disjoint_keys.0[i]));
                    // xor_filter.borrow().member(random_keys.1[i]);
                }
                return start.elapsed();
            }
            let start = Instant::now();
            for _ in 0..num_runs {
                for i in 0..(SAMPLE_SIZE as usize) {
                    // check 1st pair, i.e the same that we inserted.
                    black_box(register_aligned_bloom_filter.borrow().member(disjoint_keys.0[i]));
                }
            }
            return start.elapsed();
        });
    });
}



//criterion_group!(current_benches,setup);
criterion_group!(benches,bench_bloom_filter_fasthash_member,
    bench_blocked_bloom_filter_fasthash_member,
    bench_register_aligned_bloom_filter_fasthash_member, 
    bench_counting_bloom_filter_fasthash_member);
criterion_main!(benches);
