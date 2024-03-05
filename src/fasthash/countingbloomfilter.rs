
use rand::Rng;


#[path="../utils.rs"]
mod utils;

pub struct CountingBloomFilter {
    pub(crate) count_array: Vec<u8>,
    hash_function: (u64,u64,u64),
    num_hashes: usize,
    size: u64,
    l: u32,
}


impl CountingBloomFilter {
    pub fn new(expected_inserts: u64, false_positive_rate: f64) -> CountingBloomFilter {
        let size: u64 = ((-1.44 * (expected_inserts as f64)).ceil()
            * false_positive_rate.log2() + 0.5) as u64 ;
        let num_hashes = (-false_positive_rate.log2() + 0.5) as usize;
        CountingBloomFilter {
            count_array: vec![0; size as usize],
            hash_function: Self::generate_hash_functions(num_hashes, size),
            num_hashes,
            size,
            l: 64 - (size - 1).leading_zeros(),
        }
    }

    fn generate_hash_functions(n: usize, _m: u64) -> (u64, u64,u64) {
        let mut rng = rand::thread_rng();
        let a1: u64 = rng.gen_range(1..=u64::MAX);
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
            let index = ((h1 as u32 * i as u32 + h2 as u32) % self.size as u32) as usize;
            self.count_array[index] = self.count_array[index].saturating_add(1);
        }
    }

    pub fn member(&self, key: u64) -> bool {
        let hash = (utils::hash(key, self.l, self.hash_function.0,self. hash_function.1, self.hash_function.2) % self.size as u32);
        let h1: u16 = ((hash >> 16) & 0xFFFF) as u16;
        let h2: u16 = (hash & 0xFFFF) as u16;
        for i in 0..self.num_hashes {
            let index = ((h1 as u32 * i as u32 + h2 as u32) % self.size as u32) as usize;
            if self.count_array[index] <= 0 {
                return false;
            }
        }
        return true
    }

    pub(crate) fn delete(&mut self, key: u64) {
        let hash = (utils::hash(key, self.l, self.hash_function.0,self. hash_function.1, self.hash_function.2) % self.size as u32);
        let h1: u16 = ((hash >> 16) & 0xFFFF) as u16;
        let h2: u16 = (hash & 0xFFFF) as u16;
        for i in 0..self.num_hashes {
            let index = ((h1 as u32 * i as u32 + h2 as u32) % self.size as u32) as usize;
            self.count_array[index] = self.count_array[index].saturating_sub(1);
        }

    }
}

fn main() {

}

