use std::hash::{Hash};
use fastmurmur3;
use chrono::Utc;
use rand::Rng;

pub(crate) struct CuckooFilter {
    pub(crate) buckets: Vec<u64>,
    bucket_count: usize,
    max_kicks: usize,
    l: u32,
    hash_coefficients: (u64,u64,u64),
    full: bool,
}

impl CuckooFilter {
    pub(crate) fn new(bucket_count: usize, max_kicks: usize) -> Self {
        CuckooFilter {
            buckets: vec![0; bucket_count],
            bucket_count,
            max_kicks,
            l: 64 - (bucket_count - 1).leading_zeros(),
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
    
    fn fingerprint(key: u64) -> u32 {
        let seed = Utc::now().timestamp() as u32;
        return fastmurmur3::hash(&key.to_ne_bytes()) as u32;
        //return murmur3_x64_128(&mut b"{key}", seed).unwrap() as u32;
    }

    fn hash1(x: u64, l: u32, a1: u64, a2: u64, b: u64) -> u32 {
        let intermediate = (a1.wrapping_add(x)).wrapping_mul(a2.wrapping_add((x >> 32))).wrapping_add(b);
        let result = (intermediate) >> (64 - l);
        return result as u32;
    }

    fn hash2(&self, i_1: u32, f: u32) -> u32 {
        return i_1 ^ Self::hash1(f as u64, self.l, self.hash_coefficients.0, self.hash_coefficients.1, self.hash_coefficients.2);
    }

    pub(crate) fn insert(&mut self, key: u64) -> bool {
        let f = Self::fingerprint(key);
        let i_1 = Self::hash1(key,self.l, self.hash_coefficients.0, self.hash_coefficients.1, self.hash_coefficients.2) % self.bucket_count as u32;
        let i_2 = Self::hash2(self,i_1, f) % self.bucket_count as u32;

        if self.buckets[i_1 as usize] != 0 {
            if  self.buckets[i_2 as usize] != 0 {
                // Both buckets are occupied, perform cuckoo eviction
                for _ in 0..self.max_kicks {
                    let random_index = if rand::random() { i_1 } else { i_2 };
                    let kicked_key = std::mem::replace(&mut self.buckets[random_index as usize], key);

                    let newf = Self::fingerprint(kicked_key);
                    let newi_1 = Self::hash1(kicked_key,self.l, self.hash_coefficients.0, self.hash_coefficients.1, self.hash_coefficients.2);
                    let newi_2 = Self::hash2(self, newi_1, newf);

                    if random_index == i_1 {
                        if newi_2 != i_1 && self.buckets[newi_2 as usize] == 0 {
                            self.buckets[i_1 as usize] = 0;
                            self.buckets[newi_2 as usize] = kicked_key;
                            return true;
                        }
                    } else {
                        if newi_1 != i_2 && self.buckets[newi_1 as usize] == 0 {
                            self.buckets[i_2 as usize] = 0;
                            self.buckets[newi_1 as usize] = kicked_key;
                            return true;
                        }
                    }
                }
                return false; // Failed to evict after maximum kicks
            } else {
                self.buckets[i_2 as usize] = key;
                return true;
            }
        } else {
            self.buckets[i_1 as usize] = key;
            return true;
        }
    }

    pub(crate) fn member(&self, key: u64) -> bool {
        let f = Self::fingerprint(key);
        let i_1 = Self::hash1(key,self.l, self.hash_coefficients.0, self.hash_coefficients.1, self.hash_coefficients.2) % self.bucket_count as u32;
        let i_2 = Self::hash2(self,i_1, f) % self.bucket_count as u32;
        return self.buckets[i_1 as usize] == key || self.buckets[i_2 as usize] == key
    }
}

fn main() {

}
