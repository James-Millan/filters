use std::cell::RefCell;
use std::time::Instant;
use criterion::{black_box, Criterion, criterion_group, criterion_main};

#[path = "../mortonfilter.rs"]
mod mortonfilter;

#[path = "../keygenerator.rs"]
mod keygenerator;

static  SAMPLE_SIZE: u64 = keygenerator::SAMPLE_SIZE;


fn bench_morton_filter_uniform_member(c: &mut Criterion) {
    // setup
    let morton_filter = RefCell::new(mortonfilter::MortonFilter::new(SAMPLE_SIZE,0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let disjoint_keys = keys.disjoint;

    for j in disjoint_keys.0.clone() {
        morton_filter.borrow_mut().insert(j);
    }

    // custom benchmarking function.
    c.bench_function("bench_morton_filter_uniform_member", |b| {
        b.iter_custom(|iters| {
            let mut num_runs = (iters as f64 / SAMPLE_SIZE as f64).ceil() as u64;
            if num_runs < 1 {
                num_runs = 1;
            }
            let start = Instant::now();
            for _ in 0..num_runs {
                for i in 0..(SAMPLE_SIZE as usize) {
                    // check 1st pair, i.e the same that we inserted.
                    black_box(morton_filter.borrow().member(disjoint_keys.0[i]));
                }
            }
            return start.elapsed();
        });
    });
}

fn bench_morton_filter_mixed_member(c: &mut Criterion) {
    // setup
    let morton_filter = RefCell::new(mortonfilter::MortonFilter::new(SAMPLE_SIZE,0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let mixed_keys = keys.mixed;

    for j in mixed_keys.0 {
        morton_filter.borrow_mut().insert(j);
    }

    // custom benchmarking function.
    c.bench_function("bench_morton_filter_mixed_member", |b| {
        b.iter_custom(|iters| {
            let mut num_runs = (iters as f64 / SAMPLE_SIZE as f64).ceil() as u64;
            if num_runs < 1 {
                num_runs = 1;
            }
            let start = Instant::now();
            for _ in 0..num_runs {
                for i in 0..(SAMPLE_SIZE as usize) {
                    black_box(morton_filter.borrow().member(mixed_keys.1[i]));
                }
            }
            return start.elapsed();
        });
    });
}


fn bench_morton_filter_disjoint_member(c: &mut Criterion) {
    // setup
    let morton_filter = RefCell::new(mortonfilter::MortonFilter::new(SAMPLE_SIZE,0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let disjoint_keys = keys.disjoint;

    for j in disjoint_keys.0 {
        morton_filter.borrow_mut().insert(j);
    }

    // custom benchmarking function.
    c.bench_function("bench_morton_filter_disjoint_member", |b| {
        b.iter_custom(|iters| {
            let mut num_runs = (iters as f64 / SAMPLE_SIZE as f64).ceil() as u64;
            if num_runs < 1 {
                num_runs = 1;
            }
            let start = Instant::now();
            for _ in 0..num_runs {
                for i in 0..(SAMPLE_SIZE as usize) {
                    black_box(morton_filter.borrow().member(disjoint_keys.1[i]));
                }
            }
            return start.elapsed();
        });
    });
}
fn bench_morton_filter_random_member(c: &mut Criterion) {

    // setup
    let morton_filter = RefCell::new(mortonfilter::MortonFilter::new(SAMPLE_SIZE,0.01));
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let random_keys = keys.random;

    for j in random_keys.0 {
        morton_filter.borrow_mut().insert(j);
    }

    // custom benchmarking function.
    c.bench_function("bench_morton_filter_random_member", |b| {
        b.iter_custom(|iters| {
            let mut num_runs = (iters as f64 / SAMPLE_SIZE as f64).ceil() as u64;
            if num_runs < 1 {
                num_runs = 1;
            }
            let start = Instant::now();
            for _ in 0..num_runs {
                for i in 0..(SAMPLE_SIZE as usize) {
                    black_box(morton_filter.borrow().member(random_keys.1[i]));
                    // morton_filter.borrow().member(random_keys.1[i]);
                }
            }
            return start.elapsed();
        });
    });
}

fn bench_morton_filter_create(c: &mut Criterion) {
    // setup
    let mut keys = keygenerator::KeyGenerator::new_empty();
    keys.read_from_file().expect("");
    let random_keys = keys.random;

    // custom benchmarking function.
    c.bench_function("bench_morton_filter_create", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let mut morton_filter = mortonfilter::MortonFilter::new(SAMPLE_SIZE, 0.01);
                for i in 0..((SAMPLE_SIZE as f64 * 0.95).floor() as usize) {
                    black_box(morton_filter.insert(random_keys.1[i]));
                }
            }
            return start.elapsed();
        });
    });
}


criterion_group!(benches, bench_morton_filter_uniform_member, bench_morton_filter_disjoint_member,
    bench_morton_filter_mixed_member, bench_morton_filter_random_member, bench_morton_filter_create);
criterion_main!(benches);