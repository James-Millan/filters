
use rand::Rng;


#[path="utils.rs"]
mod utils;

pub(crate) struct CountingBloomFilter {
    pub(crate) count_array: Vec<u8>,
    hash_functions: Vec<(u64,u64,u64)>,
    size: u64,
    l: u32,
}


impl CountingBloomFilter {
    pub(crate) fn new(expected_inserts: u64, false_positive_rate: f64) -> CountingBloomFilter {
        let size: u64 = ((-1.44 * (expected_inserts as f64)).ceil()
            * false_positive_rate.log2() + 0.5) as u64 ;
        let num_hashes = (-false_positive_rate.log2() + 0.5) as usize;
        CountingBloomFilter {
            count_array: vec![0; size as usize],
            hash_functions: Self::generate_hash_functions(num_hashes, size),
            size,
            l: 64 - (size - 1).leading_zeros(),
        }
    }

    fn generate_hash_functions(n: usize, _m: u64) -> Vec<(u64, u64,u64)> {
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



    // insert hashes the key for all hash functions and sets them to be true.
    // requires a mutable reference to itself. and a reference to the key.
    pub(crate) fn insert(&mut self, key: u64) {
        if key >= self.size {
            return
        }
        for hash_function in &self.hash_functions {
            let index: usize = (utils::hash(key, self.l, hash_function.0, hash_function.1, hash_function.2) % self.size as u32) as usize;
            self.count_array[index] = self.count_array[index].saturating_add(1);
        }
    }

    pub(crate) fn member(&self, key: u64) -> bool {
        for hash_function in &self.hash_functions {
            let index: usize = (utils::hash(key, self.l, hash_function.0, hash_function.1, hash_function.2) % self.size as u32) as usize;
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
            let index: usize = (utils::hash(key, self.l, hash_function.0, hash_function.1, hash_function.2) % self.size as u32) as usize;
            self.count_array[index] = self.count_array[index].saturating_sub(1);
        }

    }
}

fn main() {

}

