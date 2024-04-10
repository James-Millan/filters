use std::cell::RefCell;
use std::time::Instant;
use criterion::{black_box, Criterion, criterion_group, criterion_main};

#[path = "../XorFilter8.rs"]
mod xorfilter8;
#[path = "../xorfilter.rs"]
mod xorfilter32;
#[path = "../threewisebinaryfusefilter8.rs"]
mod binary3fusefilter8;
#[path = "../threewisebinaryfusefilter16.rs"]
mod binary3fusefilter16;
#[path = "../threewisebinaryfusefilter32.rs"]
mod binary3fusefilter32;
#[path = "../fourwisebinaryfusefilter8.rs"]
mod binary4fusefilter8;
#[path = "../fourwisebinaryfusefilter16.rs"]
mod binary4fusefilter16;
#[path = "../fourwisebinaryfusefilter32.rs"]
mod binary4fusefilter32;

#[path = "../keygenerator.rs"]
mod keygenerator;

static  SAMPLE_SIZE: u64 = keygenerator::SAMPLE_SIZE;


fn bench_xor8_filter_create(c: &mut Criterion) {
    // custom benchmarking function.
    c.bench_function("bench_xor8_filter_create", |b| {
        b.iter_custom(|iters| {
            // setup
            let mut keys = keygenerator::KeyGenerator::new_empty();
            keys.read_from_file().expect("");
            let random_keys = keys.random;

            //benching
            let start = Instant::now();
            let xor_filter = xorfilter8::XorFilter::new(random_keys.0);
            return start.elapsed();
        });
    });
}

fn bench_xor32_filter_create(c: &mut Criterion) {
    // custom benchmarking function.
    c.bench_function("bench_xor32_filter_create", |b| {
        b.iter_custom(|iters| {
            // setup
            let mut keys = keygenerator::KeyGenerator::new_empty();
            keys.read_from_file().expect("");
            let random_keys = keys.random;

            //benching
            let start = Instant::now();
            let xor_filter = xorfilter32::XorFilter::new(random_keys.0);
            return start.elapsed();
        });
    });
}

fn bench_binary3_fuse_filter32_create(c: &mut Criterion) {
    // custom benchmarking function.
    c.bench_function("bench_binary3_fuse_filter32_create", |b| {
        b.iter_custom(|iters| {
            // setup
            let mut keys = keygenerator::KeyGenerator::new_empty();
            keys.read_from_file().expect("");
            let random_keys = keys.random;

            //benching
            let start = Instant::now();
            let binary_fuse_filter = binary3fusefilter32::ThreeWiseBinaryFuseFilter32::new(random_keys.0);
            return start.elapsed();
        });
    });
}

fn bench_binary3_fuse_filter16_create(c: &mut Criterion) {
    // custom benchmarking function.
    c.bench_function("bench_binary3_fuse_filter16_create", |b| {
        b.iter_custom(|iters| {
            // setup
            let mut keys = keygenerator::KeyGenerator::new_empty();
            keys.read_from_file().expect("");
            let random_keys = keys.random;

            //benching
            let start = Instant::now();
            let binary_fuse_filter = binary3fusefilter16::ThreeWiseBinaryFuseFilter16::new(random_keys.0);
            return start.elapsed();
        });
    });
}

fn bench_binary3_fuse_filter8_create(c: &mut Criterion) {
    // custom benchmarking function.
    c.bench_function("bench_binary3_fuse_filter8_create", |b| {
        b.iter_custom(|iters| {
            // setup
            let mut keys = keygenerator::KeyGenerator::new_empty();
            keys.read_from_file().expect("");
            let random_keys = keys.random;

            //benching
            let start = Instant::now();
            let binary_fuse_filter = binary3fusefilter8::ThreeWiseBinaryFuseFilter8::new(random_keys.0);
            return start.elapsed();
        });
    });
}

fn bench_binary4_fuse_filter32_create(c: &mut Criterion) {
    // custom benchmarking function.
    c.bench_function("bench_binary4_fuse_filter32_create", |b| {
        b.iter_custom(|iters| {
            // setup
            let mut keys = keygenerator::KeyGenerator::new_empty();
            keys.read_from_file().expect("");
            let random_keys = keys.random;

            //benching
            let start = Instant::now();
            let binary_fuse_filter = binary4fusefilter32::FourWiseBinaryFuseFilter32::new(&random_keys.0);
            return start.elapsed();
        });
    });
}

fn bench_binary4_fuse_filter16_create(c: &mut Criterion) {
    // custom benchmarking function.
    c.bench_function("bench_binary4_fuse_filter16_create", |b| {
        b.iter_custom(|iters| {

            // setup
            let mut keys = keygenerator::KeyGenerator::new_empty();
            keys.read_from_file().expect("");
            let random_keys = keys.random;

            //benching
            let start = Instant::now();
            let binary_fuse_filter = binary4fusefilter16::FourWiseBinaryFuseFilter16::new(&random_keys.0);
            return start.elapsed();
        });
    });
}

fn bench_binary4_fuse_filter8_create(c: &mut Criterion) {
    // custom benchmarking function.
    c.bench_function("bench_binary4_fuse_filter8_create", |b| {
        b.iter_custom(|iters| {
            // setup
            let mut keys = keygenerator::KeyGenerator::new_empty();
            keys.read_from_file().expect("");
            let random_keys = keys.random;

            //benching
            let start = Instant::now();
            let binary_fuse_filter = binary4fusefilter8::FourWiseBinaryFuseFilter8::new(&random_keys.0);
            return start.elapsed();
        });
    });
}


criterion_group!(benches, bench_xor8_filter_create, bench_xor32_filter_create, bench_binary3_fuse_filter8_create,
    bench_binary3_fuse_filter16_create,bench_binary3_fuse_filter32_create, bench_binary4_fuse_filter8_create,
    bench_binary4_fuse_filter16_create, bench_binary4_fuse_filter32_create);
criterion_main!(benches);