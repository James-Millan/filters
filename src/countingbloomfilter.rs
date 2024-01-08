use std::hash::{Hash};
use rand::Rng;
pub(crate) struct CountingBloomFilter {
    pub(crate) count_array: Vec<u8>,
    hash_functions: Vec<(u64,u64,u64)>,
    size: u64,
    l: u32,
}


impl CountingBloomFilter {
    pub(crate) fn new(size: u64, num_hashes: usize) -> CountingBloomFilter {
        CountingBloomFilter {
            count_array: vec![0; size as usize],
            hash_functions: Self::generate_hash_functions(num_hashes, size),
            size,
            l: 64 - (size - 1).leading_zeros(),
        }
    }

    fn generate_hash_functions(n: usize, m: u64) -> Vec<(u64, u64,u64)> {
        let mut rng = rand::thread_rng();
        let mut hash_functions = Vec::new();
        for _ in 0..n {
            let a1: u64 = rng.gen_range(1..=u64::MAX );
            let a2: u64 = rng.gen_range(1..=u64::MAX);
            let b: u64 = rng.gen_range(1..=u64::MAX);
            hash_functions.push((a1,a2,b));
        }
        return hash_functions;
    }

    // x is key to be hashed. l is binary log of filter size. a1,a2,b random u64s.
    fn hash(x: u64, l: u32, a1: u64, a2: u64, b: u64) -> u32 {
        let intermediate = (a1.wrapping_add(x)).wrapping_mul(a2.wrapping_add((x >> 32))).wrapping_add(b);
        let result = (intermediate) >> (64 - l);
        return result as u32
    }

    // insert hashes the key for all hash functions and sets them to be true.
    // requires a mutable reference to itself. and a reference to the key.
    pub(crate) fn insert(&mut self, key: u64) {
        if key >= self.size {
            return
        }
        for hash_function in &self.hash_functions {
            let index: usize = (Self::hash(key, self.l, hash_function.0, hash_function.1, hash_function.2) % self.size as u32) as usize;
            self.count_array[index] = self.count_array[index].saturating_add(1);
        }
    }

    pub(crate) fn member(&self, key: u64) -> bool {
        if key >= self.size {
            return false
        }
        for hash_function in &self.hash_functions {
            let index: usize = (Self::hash(key, self.l, hash_function.0, hash_function.1, hash_function.2) % self.size as u32) as usize;
            if self.count_array[index] <= 0 {
                return false;
            }
        }
        true
    }

    pub(crate) fn delete(&mut self, key: u64) {
        if key >= self.size {
            return
        }
        for hash_function in &self.hash_functions {
            let index: usize = (Self::hash(key, self.l, hash_function.0, hash_function.1, hash_function.2) % self.size as u32) as usize;
            self.count_array[index] = self.count_array[index].saturating_sub(1);
        }

    }
}

fn main() {

}

