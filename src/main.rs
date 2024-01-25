mod bloomfilter;
mod cuckoofilter;
mod countingbloomfilter;
mod xorfilter;
mod bitvector;
mod blockedbloomfilter;
mod utils;

extern crate rand;
use rand::Rng;

fn main() {
    let mut blocked_bloom_filter = blockedbloomfilter::BlockedBloomFilter::new(1000, 512, 0.01);
    for i in 0..=100 {
        blocked_bloom_filter.insert(i);
    }

    for j in 0..=100 {
        println!("Contains '{}': {}", j, blocked_bloom_filter.member(j));
    }
    println!("Contains '112': {}", blocked_bloom_filter.member(112));
    //println!("Buckets: {:?}", cuckoo_filter.buckets);


}
