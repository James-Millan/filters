
use fastmurmur3;

use rand::Rng;
#[path = "../bitvector.rs"]
mod bitvector;

#[path = "../utils.rs"]
mod utils;

#[path = "tabulationhashing.rs"]
mod tabulationhashing;
use tabulationhashing::TabulationHashing;

pub(crate) struct CuckooFilter {
    pub(crate) buckets: Vec<Vec<u8>>,
    bucket_count: usize,
    bucket_size: usize,
    max_kicks: usize,
    hasher: TabulationHashing,
    full: bool,
}

impl CuckooFilter {
    pub(crate) fn new(bucket_count: usize, max_kicks: usize, bucket_size: usize) -> Self {
        CuckooFilter {
            buckets: vec![vec![0; bucket_size]; bucket_count],
            bucket_count,
            bucket_size,
            max_kicks,
            hasher: tabulationhashing::TabulationHashing::new(),
            full: false
        }
    }
    
    fn fingerprint(key: u64) -> u8 {
        return fastmurmur3::hash(&key.to_ne_bytes()) as u8;
        //return murmur3_x64_128(&mut b"{key}", seed).unwrap() as u32;
    }
    fn hash2(&self, i_1: u32, f: u32) -> u32 {
        return i_1 ^ self.hasher.tabulation_hashing(i_1 as u64) as u32;
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
        let i_1 = (self.hasher.tabulation_hashing(key) % self.bucket_count as u64) as u32;
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
            let new_i = (random_bucket ^ self.hasher.tabulation_hashing(f) as u32)
                % self.bucket_count as u32;

            // try inserting kicked_key into new_i
            if Self::find_empty_and_set(self, new_i as usize, f as u8) {
                //println!("finally inserted, after: '{}' kicks", count);
                return true;
            }
        }
        // Failed to evict after maximum kicks
        self.full = true;
        return false;
    }

    pub fn member(&self, key: u64) -> bool {
        let f = Self::fingerprint(key) as u64;
        let i_1 = (self.hasher.tabulation_hashing(key) % self.bucket_count as u64) as u32;
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
        let i_1 = (self.hasher.tabulation_hashing(key) % self.bucket_count as u64) as u32;
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
