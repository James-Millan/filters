
use rand::Rng;
use std::f64;
use std::hash::Hash;

#[path = "../bitvector.rs"]
mod bitvector;

#[path = "../utils.rs"]
mod utils;

#[path = "tabulationhashing.rs"]
mod tabulationhashing;

use tabulationhashing::TabulationHashing;

pub struct BloomFilter {
    pub(crate) bit_array: bitvector::BitVector,
    pub(crate) hash_functions: Vec<TabulationHashing>,
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
            hash_functions: Self::generate_hash_functions(num_hashes, size),
            size,
            l: utils::log_base(size as f64, 2f64) as u32,
        }
    }

    fn generate_hash_functions(n: usize, _m: u64) -> Vec<TabulationHashing> {
        let mut hash_functions = Vec::new();
        for _ in 0..n {
            hash_functions.push(TabulationHashing::new());
        }
        return hash_functions;
    }



    // insert hashes the key for all hash functions and sets them to be true.
    // requires a mutable reference to itself. and a reference to the key.
    pub fn insert(&mut self, key: u64) {
        for hash_function in &self.hash_functions {
            let index : u64 = (hash_function.tabulation_hashing(key) % self.size);
            // println!("{}", index);
            // println!("{}", Self::hash( key, self.l, hash_function.0, hash_function.1, hash_function.2) );
            self.bit_array.insert(index);
        }
    }

    pub(crate) fn member(&self, key: u64) -> bool {
        for hash_function in &self.hash_functions {
            let index : u64 = (hash_function.tabulation_hashing(key) % self.size);
            if !self.bit_array.member(index) {
                return false;
            }
        }
        return true;
    }
}