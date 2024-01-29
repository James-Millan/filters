mod bloomfilter;
mod cuckoofilter;
mod countingbloomfilter;
mod xorfilter;
mod bitvector;
mod blockedbloomfilter;
mod utils;
mod dleftcountingfilter;
mod registeralignedbloomfilter;

extern crate rand;

use std::cell::RefCell;
use rand::Rng;
use crate::cuckoofilter::CuckooFilter;

fn main() {
    let sample_size = 100000000;
    let mut cuckoo_filter =CuckooFilter::new(100000, 100000, 8);
    for i in 0..=100000 {
        cuckoo_filter.insert(i as u64);
    }
    //println!("Buckets: {:?}", cuckoo_filter.buckets);

    // let mut register_aligned_bloom_filter = registeralignedbloomfilter::RegisterAlignedBloomFilter::new(
    //     1000, 8, 0.01);
    // for i in 0..=100 {
    //     register_aligned_bloom_filter.insert(i);
    // }
    //
    // for j in 0..=100 {
    //     println!("Contains '{}': {}", j, register_aligned_bloom_filter.member(j));
    // }
    // println!("Contains '112': {}", register_aligned_bloom_filter.member(112));
    // //println!("Buckets: {:?}", cuckoo_filter.buckets);



}
