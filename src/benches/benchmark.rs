use std::cell::RefCell;
use std::fmt::Debug;
use criterion::{Criterion};
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use bloomfilter::BloomFilter;
use crate::countingbloomfilter::CountingBloomFilter;
use crate::cuckoofilter::CuckooFilter;

#[path = "../bloomfilter.rs"]
mod bloomfilter;

#[path = "../cuckoofilter.rs"]
mod cuckoofilter;

#[path = "../countingbloomfilter.rs"]
mod countingbloomfilter;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter() {
        let mut bloomfilter = BloomFilter::new(10000, 6);
        for i in 0..1000{
            bloomfilter.insert(i);
            assert_eq!(bloomfilter.member(i), true);
        }
    }
    #[test]
    fn test_cuckoo_filter() {
        let mut cuckoofilter = crate::cuckoofilter::CuckooFilter::new(10000, 100);
        for i in 0..1000{
            cuckoofilter.insert(i);
            assert_eq!(cuckoofilter.member(i), true);
        }
    }

    #[test]
    fn test_counting_bloom_filter() {
        let mut countingbloomfilter = crate::countingbloomfilter::CountingBloomFilter::new(10000, 6);
        for i in 0..1000{
            countingbloomfilter.insert(i);
            assert_eq!(countingbloomfilter.member(i), true);
        }
    }
}
fn bench_bloom_filter_create(c: &mut Criterion) {
    c.bench_function("bench_bloom_filter_create", |b| {
        b.iter(|| {
            let mut bloom_filter = BloomFilter::new(100000, 6);
            //stop it being optimized by the compiler
            black_box(bloom_filter);
        });
    });
}

fn bench_bloom_filter_insert(c: &mut Criterion) {
    let sample_size = 1000000000;
    let bloom_filter = RefCell::new(BloomFilter::new(sample_size, 100));

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

fn bench_cuckoo_filter_create(c: &mut Criterion) {
    c.bench_function("bench_cuckoo_filter_create", |b| {
        b.iter(|| {
            let mut cuckoo_filter = CuckooFilter::new(100000, 6);
            //stop it being optimized by the compiler
            black_box(cuckoo_filter);
        });
    });
}

fn bench_cuckoo_filter_insert(c: &mut Criterion) {
    let sample_size = 1000000000;
    let cuckoo_filter = RefCell::new(CuckooFilter::new(sample_size, 100));

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

fn bench_counting_bloom_filter_create(c: &mut Criterion) {
    c.bench_function("bench_counting_bloom_filter_create", |b| {
        b.iter(|| {
            let mut counting_bloom__filter = CountingBloomFilter::new(100000, 6);
            //stop it being optimized by the compiler
            black_box(counting_bloom__filter);
        });
    });
}

fn bench_counting_bloom_filter_insert(c: &mut Criterion) {
    let sample_size = 1000000000;
    let counting_bloom_filter = RefCell::new(CountingBloomFilter::new(sample_size, 100));

    //let mut bloom_filter = BloomFilter::new(100000, 6).clone();
    let mut i = 0;
    c.bench_function("bench_counting_bloom_filter_insert", |b| {
        b.iter(|| {
            counting_bloom_filter.borrow_mut().insert(i);
            i += 1;
            //stop it being optimized by the compiler
            //black_box(bloom_filter);
        });
    });
}

criterion_group!(benches, bench_bloom_filter_create, bench_bloom_filter_insert, bench_cuckoo_filter_create, bench_cuckoo_filter_insert,
                bench_counting_bloom_filter_insert, bench_counting_bloom_filter_create);
criterion_main!(benches);
