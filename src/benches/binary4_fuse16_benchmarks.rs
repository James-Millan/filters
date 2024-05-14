use std::cell::RefCell;
use std::time::Instant;
use criterion::{black_box, Criterion, criterion_group, criterion_main};

#[path = "../fourwisebinaryfusefilter16.rs"]
mod binaryfusefilter;

#[path = "../keygenerator.rs"]
mod keygenerator;

static  SAMPLE_SIZE: u64 = keygenerator::SAMPLE_SIZE;


fn bench_binary4_fuse16_filter_uniform_member(c: &mut Criterion) {
    // setup
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let disjoint_keys = keys.disjoint;
    let binary_fuse_filter = RefCell::new(binaryfusefilter::FourWiseBinaryFuseFilter16::new(&disjoint_keys.0.clone()));

    // custom benchmarking function.
    c.bench_function("bench_binary4_fuse16_filter_uniform_member", |b| {
        b.iter_custom(|iters| {
            let mut num_runs = (iters as f64 / SAMPLE_SIZE as f64).ceil() as u64;
            let remainder = iters % SAMPLE_SIZE;
            if num_runs <= 1 {
                //  just run the for loop form 0 - iters in here.
                let start = Instant::now();
                for i in 0..(iters as usize) {
                    black_box(binary_fuse_filter.borrow().member(disjoint_keys.0[i]));
                    // xor_filter.borrow().member(random_keys.1[i]);
                }
                return start.elapsed();            
            }
            else {
                num_runs -= 1;
                let start = Instant::now();
                for _ in 0..num_runs {
                    for i in 0..(SAMPLE_SIZE as usize) {
                        // check 1st pair, i.e the same that we inserted.
                        black_box(binary_fuse_filter.borrow().member(disjoint_keys.0[i]));
                    }
                }
                for i in 0..(remainder as usize) {
                    black_box(binary_fuse_filter.borrow().member(disjoint_keys.0[i]));
                }
                return start.elapsed();
            }
            
        });
    });
}
fn bench_binary4_fuse16_filter_mixed_member(c: &mut Criterion) {
    // setup
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let mixed_keys = keys.mixed;
    let binary_fuse_filter = RefCell::new(binaryfusefilter::FourWiseBinaryFuseFilter16::new(&mixed_keys.0.clone()));

    // custom benchmarking function.
    c.bench_function("bench_binary4_fuse16_filter_mixed_member", |b| {
        b.iter_custom(|iters| {
            let mut num_runs = (iters as f64 / SAMPLE_SIZE as f64).ceil() as u64;
            let remainder = iters % SAMPLE_SIZE;
            if num_runs <= 1 {
                //  just run the for loop form 0 - iters in here.
                let start = Instant::now();
                for i in 0..(iters as usize) {
                    black_box(binary_fuse_filter.borrow().member(mixed_keys.0[i]));
                    // xor_filter.borrow().member(random_keys.1[i]);
                }
                return start.elapsed();            
            }
            else {
                num_runs -= 1;
                let start = Instant::now();
                for _ in 0..num_runs {
                    for i in 0..(SAMPLE_SIZE as usize) {
                        // check 1st pair, i.e the same that we inserted.
                        black_box(binary_fuse_filter.borrow().member(mixed_keys.0[i]));
                    }
                }
                for i in 0..(remainder as usize) {
                    black_box(binary_fuse_filter.borrow().member(mixed_keys.0[i]));
                }
                return start.elapsed();
            }
        });
    });
}
fn bench_binary4_fuse16_filter_disjoint_member(c: &mut Criterion) {
    // setup
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let disjoint_keys = keys.disjoint;
    let binary_fuse_filter = RefCell::new(binaryfusefilter::FourWiseBinaryFuseFilter16::new(&disjoint_keys.0.clone()));

    // custom benchmarking function.
    c.bench_function("bench_binary4_fuse16_filter_disjoint_member", |b| {
        b.iter_custom(|iters| {
            let mut num_runs = (iters as f64 / SAMPLE_SIZE as f64).ceil() as u64;
            let remainder = iters % SAMPLE_SIZE;
            if num_runs <= 1 {
                //  just run the for loop form 0 - iters in here.
                let start = Instant::now();
                for i in 0..(iters as usize) {
                    black_box(binary_fuse_filter.borrow().member(disjoint_keys.1[i]));
                    // xor_filter.borrow().member(random_keys.1[i]);
                }
                return start.elapsed();            
            }
            else {
                num_runs -= 1;
                let start = Instant::now();
                for _ in 0..num_runs {
                    for i in 0..(SAMPLE_SIZE as usize) {
                        // check 1st pair, i.e the same that we inserted.
                        black_box(binary_fuse_filter.borrow().member(disjoint_keys.1[i]));
                    }
                }
                for i in 0..(remainder as usize) {
                    black_box(binary_fuse_filter.borrow().member(disjoint_keys.1[i]));
                }
                return start.elapsed();
            }
        });
    });
}
fn bench_binary4_fuse16_filter_random_member(c: &mut Criterion) {

    // setup
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let random_keys = keys.random;
    let binary_fuse_filter = RefCell::new(binaryfusefilter::FourWiseBinaryFuseFilter16::new(&random_keys.0.clone()));

    // custom benchmarking function.
    c.bench_function("bench_binary4_fuse16_filter_random_member", |b| {
        b.iter_custom(|iters| {
            let mut num_runs = (iters as f64 / SAMPLE_SIZE as f64).ceil() as u64;
            let remainder = iters % SAMPLE_SIZE;
            if num_runs <= 1 {
                //  just run the for loop form 0 - iters in here.
                let start = Instant::now();
                for i in 0..(iters as usize) {
                    black_box(binary_fuse_filter.borrow().member(random_keys.0[i]));
                    // xor_filter.borrow().member(random_keys.1[i]);
                }
                return start.elapsed();            
            }
            else {
                num_runs -= 1;
                let start = Instant::now();
                for _ in 0..num_runs {
                    for i in 0..(SAMPLE_SIZE as usize) {
                        // check 1st pair, i.e the same that we inserted.
                        black_box(binary_fuse_filter.borrow().member(random_keys.0[i]));
                    }
                }
                for i in 0..(remainder as usize) {
                    black_box(binary_fuse_filter.borrow().member(random_keys.0[i]));
                }
                return start.elapsed();
            }
        });
    });
}


criterion_group!(benches, bench_binary4_fuse16_filter_uniform_member);
criterion_main!(benches);