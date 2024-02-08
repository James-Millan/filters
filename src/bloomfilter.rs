
use rand::Rng;
use std::f64;
#[path = "bitvector.rs"]
mod bitvector;

#[path = "utils.rs"]
mod utils;

pub(crate) struct BloomFilter {
    pub(crate) bit_array: bitvector::BitVector,
    pub(crate) hash_functions: Vec<(u64,u64,u64)>,
    size: u64,
    l: u32,
}
impl BloomFilter {
    pub(crate) fn new(expected_inserts: u64, false_positive_rate: f64) -> BloomFilter {
        let size: u64 = ((-1.44 * (expected_inserts as f64)).ceil()
            * false_positive_rate.log2() + 0.5) as u64 ;
        let num_hashes = (-false_positive_rate.log2() + 0.5) as usize;


        BloomFilter {
            bit_array: bitvector::BitVector::new(size),
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
            let index : u64 = (utils::hash(key, self.l, hash_function.0, hash_function.1, hash_function.2) % self.size as u32) as u64;
            // println!("{}", index);
            // println!("{}", Self::hash( key, self.l, hash_function.0, hash_function.1, hash_function.2) );
            self.bit_array.insert(index);
        }
    }

    pub(crate) fn member(&mut self, key: u64) -> bool {
        if key >= self.size {
            return false
        }
        for hash_function in &self.hash_functions {
            let index: u64 = (utils::hash(key, self.l, hash_function.0, hash_function.1, hash_function.2) % self.size as u32) as u64;
            //println!("{}", index);
            let mem = self.bit_array.clone().member(index);
            if !mem {
                return false;
            }
        }
        //println!("Hash functions: {:?}", self.hash_functions);
        //println!("Bit Array: {:?}", self.bit_array);
        true
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
