use std::collections::{binary_heap, HashSet};
use rand::{random, Rng};
use crate::blockedbloomfilter::BlockedBloomFilter;
use crate::bloomfilter::BloomFilter;
use crate::countingbloomfilter::CountingBloomFilter;
use crate::registeralignedbloomfilter::RegisterAlignedBloomFilter;

pub(crate) fn bloom_filter_fpr(size: u64, fpr:f64, keys: &Vec<u64>, lookup_keys: &Vec<u64>) {
    let mut bloom_filter = BloomFilter::new(size, fpr);
    for key in keys {
        bloom_filter.insert(*key);
    }
        let mut count: f64 = 0f64;
    let mut fp: f64 = 0f64;
    for i in lookup_keys {
        count += 1.0f64;
        if (bloom_filter.member(*i)) {
            fp += 1.0f64;
        }
    }
    let fpr = fp/count;
    println!("Bloom filter fpr: '{}'", fpr);
}

pub(crate) fn counting_bloom_filter_fpr(size: u64, fpr:f64, keys: &Vec<u64>, lookup_keys: &Vec<u64>) {
    let mut counting_bloom_filter = CountingBloomFilter::new(size, fpr);
    for key in keys {
        counting_bloom_filter.insert(*key);
    }
    let mut count: f64 = 0f64;
    let mut fp: f64 = 0f64;
    for i in lookup_keys {
        count += 1.0f64;
        if (counting_bloom_filter.member(*i)) {
            fp += 1.0f64;
        }
    }
    let fpr = fp/count;
    println!("Counting Bloom filter fpr: '{}'", fpr);
}
pub(crate) fn blocked_bloom_filter_fpr(size: u64, fpr:f64, keys: &Vec<u64>, lookups : &Vec<u64>) {
    let mut blocked_bloom_filter = BlockedBloomFilter::new(size, 512, fpr);
    for key in keys {
        blocked_bloom_filter.insert(*key);
    }
    let mut count: f64 = 0f64;

    let mut sum = 0f64;
    for j in 0..100 {
        let mut fp: f64 = 0f64;
        for i in lookups {
            count += 1.0f64;
            if (blocked_bloom_filter.member(*i)) {
                fp += 1.0f64;
            }
        }
        sum += fp/count;
    }
    println!("Blocked Bloom filter fpr: '{}'", sum/100.0);
}

pub(crate) fn register_aligned_bloom_filter_fpr(size: u64, fpr:f64, keys: &Vec<u64>, lookup_keys: &Vec<u64>) {
    let mut register_aligned_bloom_filter = RegisterAlignedBloomFilter::new(size, 64, fpr);
    for key in keys {
        register_aligned_bloom_filter.insert(*key);
    }
    let mut count: f64 = 0f64;
    let mut fp: f64 = 0f64;
    for i in lookup_keys {
        count += 1.0f64;
        if (register_aligned_bloom_filter.member(*i)) {
            fp += 1.0f64;
        }
    }
    let fpr = fp / count;
    println!("Register Aligned Bloom filter fpr: '{}'", fpr);
}

pub(crate) fn run_fpr_tests(size: u64) {
    println!("fast hashing normal");
    println!("{}", size);
    let mut keys = Vec::new();
    for i in 0..=size {
        keys.push(i);
        // println!("'{}', '{}'",hasher.tabulation_hashing(i), i);
    }
    let mut lookup_keys: Vec<u64> = Vec::new();
    for i in size+1..(size as f64*2.5f64) as u64 {
        lookup_keys.push(i);
    }

    blocked_bloom_filter_fpr(size,0.01,&keys,&lookup_keys);
    bloom_filter_fpr(size, 0.01, &keys,&lookup_keys);
    counting_bloom_filter_fpr(size,0.01,&keys,&lookup_keys);
    blocked_bloom_filter_fpr(size,0.01,&keys,&lookup_keys);
    register_aligned_bloom_filter_fpr(size,0.01,&keys,&lookup_keys);
}

pub(crate) fn run_randomised_fpr_tests(size: u64) {
    println!("fast hash random");
    println!("{}", size);
    //let mut keys = Vec::new();
    let mut set_keys = HashSet::new();
    let mut rng = rand::thread_rng();

    while set_keys.len() < size as usize {
        let random_value: u64 = rng.gen_range(0..(size as f64*2.5f64) as u64);
        if !set_keys.contains(&random_value) {
            set_keys.insert(random_value);
        }
    }

    let keys: Vec<u64> = set_keys.iter().copied().collect();

    let mut lookup_keys = HashSet::new();
    while lookup_keys.len() < size as usize {
        let random_value: u64 = rng.gen_range(0..(size as f64*2.5f64) as u64);
        if !set_keys.contains(&random_value) && !lookup_keys.contains(&random_value) {
            lookup_keys.insert(random_value);
        }
    }

    let lookup_keys: Vec<u64>  = lookup_keys.iter().copied().collect();

    blocked_bloom_filter_fpr(size,0.01,&keys,&lookup_keys);
    bloom_filter_fpr(size, 0.01, &keys,&lookup_keys);
    counting_bloom_filter_fpr(size,0.01,&keys,&lookup_keys);
    blocked_bloom_filter_fpr(size,0.01,&keys,&lookup_keys);
    register_aligned_bloom_filter_fpr(size,0.01,&keys,&lookup_keys);
}