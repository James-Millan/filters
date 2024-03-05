
use rand::Rng;
use std::f64;

#[path = "../bitvector.rs"]
mod bitvector;

#[path = "../utils.rs"]
mod utils;

pub struct BloomFilter {
    pub(crate) bit_array: bitvector::BitVector,
    pub(crate) hash_function: (u64,u64,u64),
    num_hashes: usize,
    size: u64,
    l: u32,
}
impl BloomFilter {
    pub fn new(expected_inserts: u64, false_positive_rate: f64) -> BloomFilter {
        let size: u64 = utils::closest_power_of_two(((-1.44 * (expected_inserts as f64)).ceil()
            * false_positive_rate.log2() + 0.5) as u64);
        let num_hashes = (-false_positive_rate.log2() + 0.5) as usize;


        BloomFilter {
            bit_array: bitvector::BitVector::new(size),
            hash_function: Self::generate_hash_function(num_hashes, size),
            num_hashes,
            size,
            l: utils::log_base(size as f64, 2f64) as u32,
        }
    }

    fn generate_hash_function(n: usize, _m: u64) -> (u64, u64,u64) {
        let mut rng = rand::thread_rng();
        let a1: u64 = rng.gen_range(1..=u64::MAX );
        let a2: u64 = rng.gen_range(1..=u64::MAX);
        let b: u64 = rng.gen_range(1..=u64::MAX);
        return (a1,a2,b);
    }



    // insert hashes the key for all hash functions and sets them to be true.
    // requires a mutable reference to itself. and a reference to the key.
    pub fn insert(&mut self, key: u64) {
        let hash = (utils::hash(key, self.l, self.hash_function.0,self. hash_function.1, self.hash_function.2) % self.size as u32);
        let h1: u16 = ((hash >> 16) & 0xFFFF) as u16;
        let h2: u16 = (hash & 0xFFFF) as u16;
        for i in 0..self.num_hashes {
            let index : u64 = ((h1 as u32 * i as u32 + h2 as u32) % self.size as u32) as u64;
            self.bit_array.insert(index);
        }
    }

    pub(crate) fn member(&self, key: u64) -> bool {
        let hash = (utils::hash(key, self.l, self.hash_function.0,self. hash_function.1, self.hash_function.2) % self.size as u32);
        let h1: u16 = ((hash >> 16) & 0xFFFF) as u16;
        let h2: u16 = (hash & 0xFFFF) as u16;
        for i in 0..self.num_hashes{
            let index : u64 = ((h1 as u32 * i as u32 + h2 as u32) % self.size as u32) as u64;
            if !self.bit_array.member(index) {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let num_inserts = 100;
    let fpr = 0.01;
    let mut bloom_filter = BloomFilter::new(num_inserts, fpr);

    bloom_filter.insert(23);
    bloom_filter.insert(2);
    bloom_filter.insert(11);

    println!("Contains '23': {}", bloom_filter.member(23));
    println!("Contains '12': {}", bloom_filter.member(12));
}
