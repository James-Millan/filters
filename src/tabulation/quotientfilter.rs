use rand::Rng;
#[path = "../quotientinfo.rs"]
mod quotientinfo;

#[path = "../utils.rs"]
mod utils;

use quotientinfo::QuotientInfo;
use utils::{hash};

#[path = "tabulationhashing.rs"]
mod tabulationhashing;
use tabulationhashing::TabulationHashing;

pub struct QuotientFilter {
    pub(crate) buckets: Vec<u32>,
    bucket_info: Vec<QuotientInfo>,
    hashes: Vec<TabulationHashing>,
    size: u64,
    r: u8
}

impl QuotientFilter {
    pub fn new(size: u64) -> QuotientFilter {
        let length: usize = (size) as usize;
        return QuotientFilter {
            buckets: vec![0u32; length],
            bucket_info: Self::generate_info(length),
            hashes: Self::generate_hash_functions(1),
            size,
            r: utils::log_base(size as f64, 2f64) as u8
        };
    }

    fn generate_info(size: usize) -> Vec<QuotientInfo> {
        let mut infos = Vec::new();
        for i in 0..size {
            infos.push(QuotientInfo::new());
        }
        return infos;
    }

    fn generate_hash_functions(n: u64) -> Vec<TabulationHashing> {
        let mut rng = rand::thread_rng();
        let mut hash_functions = Vec::new();
        for _ in 0..n {
            hash_functions.push(tabulationhashing::TabulationHashing::new())
        }
        return hash_functions;
    }

    /*
    To insert/delete a fingerprint f,first we mark/unmark A[f_q] as occupied.
    Next, we search for f_r using the same algorithm as May-Contain to
    find the slot where it should go. Finally, we insert/remove
    f_r and shift subsequent items as necessary, while updating
    the other two meta-data bits. We stop shifting items as soon
    as we reach an empty slot.
     */

    pub fn insert(&mut self, x: u64) -> bool {
        let fingerprint = self.fingerprint(x);
        let q = self.get_quotient(fingerprint);
        let r = self.get_remainder(fingerprint);

        let mut ins = q;

        // Find the insertion point.
        while self.bucket_info[ins as usize].is_occupied {
            if !self.bucket_info[ins as usize].is_shifted &&
                !self.bucket_info[ins as usize].is_continuation {
                // If the current bucket is occupied and not shifted or a continuation,
                // increase insertion index.
                ins += 1;
            } else if self.bucket_info[ins as usize].is_continuation {
                // Follow the continuation until the end, then insert.
                while self.bucket_info[ins as usize].is_continuation {
                    ins += 1;
                }
                break;
            } else {
                // If the bucket is shifted but not a continuation, insert at this location.
                break;
            }

            // if insertion index is out of bounds, return false.
            if !(ins < (self.size - 1) as u32) {
                return false;
            }
        }

        // we have our first attempt at an insertion index.

        // If the bucket isn't occupied and isn't a continuation, insert the fingerprint here.
        if !self.bucket_info[ins as usize].is_occupied && !self.bucket_info[ins as usize].is_continuation &&
            !self.bucket_info[ins as usize].is_shifted {
            self.buckets[ins as usize] = r;
            self.bucket_info[ins as usize].is_occupied = true;
            self.bucket_info[ins as usize].is_shifted = false;
            self.bucket_info[ins as usize].is_continuation = false;
            return true;
        } else {
            // find index of last occupied bucket.
            let mut i = ins;
            let mut last_occupied_bucket = ins.saturating_sub(1);
            while i > 0 {
                if self.bucket_info[i as usize].is_occupied {
                    last_occupied_bucket = i;
                }
                i -= 1;
            }

            // Shift elements to the right.
            while i < last_occupied_bucket {
                if self.bucket_info[i as usize].is_occupied {
                    self.buckets[i as usize] = self.buckets[(i + 1) as usize];
                    self.bucket_info[i as usize].is_shifted = true;
                    self.bucket_info[i as usize].is_occupied = self.bucket_info[(i) as usize].is_occupied;
                    self.bucket_info[i as usize].is_continuation = self.bucket_info[(i + 1) as usize].is_continuation;
                }
                i += 1;
            }

            // Insert the fingerprint into the appropriate bucket.
            self.buckets[last_occupied_bucket as usize] = r;
            self.bucket_info[last_occupied_bucket as usize].is_occupied = true;
            self.bucket_info[last_occupied_bucket as usize].is_shifted = true;
            // TODO may need to improve condition on setting this.
            self.bucket_info[last_occupied_bucket as usize].is_continuation = !(last_occupied_bucket == q);

            return true;
        }
    }

    pub fn member(&self, x: u64) -> bool {
        let f = self.fingerprint(x);
        let q = self.get_quotient(f);
        let r = self.get_remainder(f);

        if !self.bucket_info[q as usize].is_occupied {
            // there are no fingerprints in the filter that map to this bucket.
            return false;
        }
        else if self.buckets[q as usize] == r {
            // quick check. speeds up some queries
            return true;
        }
        else if self.buckets[(q + 1) as usize] == r {
            return true;
        }


        let mut b = q;
        while self.bucket_info[b as usize].is_shifted && b > 0 {
            b = b - 1;
        }

        let mut s = b;
        let mut i = 0;
        while b != q {
            if i > 10 {
                break;
            }
            while !self.bucket_info[s as usize].is_continuation && (s as u64) < self.size - 1 {
                s = s + 1;
            }
            while !self.bucket_info[b as usize].is_occupied && (b as u64) < self.size - 1{
                b = b + 1;
            }
            i += 1;
        }
        // it is possible for this to take a while.
        let mut j = 0;
        while !self.bucket_info[s as usize].is_continuation && (s as u64) < self.size - 1 {
            // println!("{}, {}", self.buckets[s as usize], r);
            j = j + 1;
            if self.buckets[s as usize] == r {
                // println!("{}", j);
                return true;
            }
            s = s + 1;
        }
        // println!("{}", j);
        if self.buckets[s as usize] == r {
            return true;
        }
        // it's not in the filter.
        return false;
    }

    fn fingerprint(&self, x: u64) -> u32 {
        return self.hashes[0].tabulation_hashing(x) as u32;
    }

    fn get_quotient(&self, fingerprint: u32) -> u32 {
        // get highest r bits
        return (fingerprint << self.r) % self.size as u32;
    }

    fn get_remainder(&self, fingerprint: u32) -> u32 {
        //get lowest r bits
        return (fingerprint & ((1 << self.r) - 1)) % self.size as u32;
    }
}