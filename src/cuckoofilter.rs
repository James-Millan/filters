
use fastmurmur3;

use rand::Rng;
use crate::cuckoofilter::utils::log_base;

#[path="utils.rs"]
mod utils;

pub(crate) struct CuckooFilter {
    pub(crate) buckets: Vec<Vec<u8>>,
    bucket_count: usize,
    bucket_size: usize,
    max_kicks: usize,
    l: u32,
    hash_coefficients: (u64,u64,u64),
    full: bool,
}

impl CuckooFilter {
    pub(crate) fn new(bucket_count: usize, max_kicks: usize, bucket_size: usize) -> Self {
        CuckooFilter {
            buckets: vec![vec![0; bucket_size]; bucket_count],
            bucket_count,
            bucket_size,
            max_kicks,
            l: log_base(bucket_count as f64, 2f64) as u32,
            hash_coefficients: Self::get_hash_coefficients(),
            full: false
        }
    }

    fn get_hash_coefficients() -> (u64,u64,u64) {
        let mut rng = rand::thread_rng();
        let a1: u64 = rng.gen_range(1..=u64::MAX );
        let a2: u64 = rng.gen_range(1..=u64::MAX);
        let b: u64 = rng.gen_range(1..=u64::MAX);
        return (a1, a2, b);
    }
    
    fn fingerprint(key: u64) -> u8 {
        return fastmurmur3::hash(&key.to_ne_bytes()) as u8;
        //return murmur3_x64_128(&mut b"{key}", seed).unwrap() as u32;
    }
    fn hash2(&self, i_1: u32, f: u32) -> u32 {
        return i_1 ^ utils::hash(f as u64, self.l, self.hash_coefficients.0, self.hash_coefficients.1, self.hash_coefficients.2);
    }
    fn find_empty_and_set(&mut self, index: usize, f: u8) -> bool {
        for j in 0..self.bucket_size {
            if self.buckets[index as usize][j] == 0 {
                self.buckets[index as usize][j] = f as u8;
                return true;
            }
        }
        return false;
    }

    pub fn insert(&mut self, key: u64) -> bool {
        let mut f = Self::fingerprint(key) as u64;
        let i_1 = utils::hash(key,self.l, self.hash_coefficients.0, self.hash_coefficients.1, self.hash_coefficients.2) % self.bucket_count as u32;
        let i_2 = Self::hash2(self,i_1, f as u32) % self.bucket_count as u32;

        // Try insert in bucket i_1
        if Self::find_empty_and_set(self, i_1 as usize, f as u8) {
            //println!("all fine");
            return true;
        }
        // Try insert in bucket i_2
        if Self::find_empty_and_set(self, i_2 as usize, f as u8) {
            //println!("all fine");
            return true;
        }

        // Both buckets are occupied, perform cuckoo eviction
        let random_bucket = if rand::random() { i_1 } else { i_2 };
        for _count in 0..self.max_kicks {
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..=self.bucket_size - 1);
            let kicked_key = std::mem::replace(&mut self.buckets[random_bucket as usize][random_index], f as u8);
            f = kicked_key as u64;
            let new_i = (random_bucket ^ utils::hash(f,self.l, self.hash_coefficients.0, self.hash_coefficients.1, self.hash_coefficients.2))
                % self.bucket_count as u32;

            // try inserting kicked_key into new_i
            if Self::find_empty_and_set(self, new_i as usize, f as u8) {
                //println!("finally inserted, after: '{}' kicks", count);
                return true;
            }
        }
        // Failed to evict after maximum kicks
        //println!("failed to insert.");
        self.full = true;
        return false;
    }

    pub fn member(&self, key: u64) -> bool {
        let f = Self::fingerprint(key) as u64;
        let i_1 = utils::hash(key,self.l, self.hash_coefficients.0, self.hash_coefficients.1, self.hash_coefficients.2) % self.bucket_count as u32;
        let i_2 = Self::hash2(self,i_1, f as u32) % self.bucket_count as u32;
        for j in 0..self.bucket_size {
            if self.buckets[i_1 as usize][j] == f as u8 || self.buckets[i_2 as usize][j] == f as u8 {
                return true
            }
        }
        return false;
    }

    pub(crate) fn delete(&mut self, key: u64) -> bool {
        let f = Self::fingerprint(key) as u64;
        let i_1 = utils::hash(key,self.l, self.hash_coefficients.0, self.hash_coefficients.1, self.hash_coefficients.2) % self.bucket_count as u32;
        let i_2 = Self::hash2(self,i_1, f as u32) % self.bucket_count as u32;
        for j in 0..self.bucket_size {
            if self.buckets[i_1 as usize][j] == f as u8 {
                self.buckets[i_1 as usize][j] = 0;
                return true;
            }
            else if self.buckets[i_2 as usize][j] == f as u8 {
                self.buckets[i_2 as usize][j] = 0;
                return true;
            }
        }
        return false;
    }
}

fn main() {

}
