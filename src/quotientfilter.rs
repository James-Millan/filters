use rand::Rng;
use crate::quotientinfo::QuotientInfo;
use crate::utils;

pub struct QuotientFilter {
    pub(crate) buckets: Vec<u32>,
    bucket_info: Vec<QuotientInfo>,
    hashes: Vec<(u64,u64,u64)>,
    size: u64,
    r: u8
}

impl QuotientFilter {
    pub(crate) fn new(size: u64) -> QuotientFilter {
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

    fn generate_hash_functions(n: u64) -> Vec<(u64, u64,u64)> {
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

    /*
    To insert/delete a fingerprint f,first we mark/unmark A[f_q] as occupied.
    Next, we search for f_r using the same algorithm as May-Contain to
    find the slot where it should go. Finally, we insert/remove
    f_r and shift subsequent items as necessary, while updating
    the other two meta-data bits. We stop shifting items as soon
    as we reach an empty slot.
     */


    // pub(crate) fn insert(&mut self, x: u64) {
    //     let f = self.fingerprint(x);
    //     let q = self.get_quotient(f);
    //     let r = self.get_remainder(f);
    //
    //     let mut s = q;
    //
    //     // Find the insertion point. Continue until end of previous run.
    //     while self.bucket_info[s as usize].is_continuation {
    //         s += 1;
    //     }
    //
    //     // Check new insertion point.
    //     if !self.bucket_info[s as usize].is_occupied && !self.bucket_info[s as usize].is_shifted &&
    //         !self.bucket_info[s as usize].is_continuation {
    //         // If the bucket is not occupied, insert the fingerprint
    //         self.buckets[s as usize] = r;
    //         self.bucket_info[s as usize].is_occupied = true;
    //         self.bucket_info[s as usize].is_shifted = false;
    //         self.bucket_info[s as usize].is_continuation = false;
    //     }
    //     else {
    //         // Find the next available empty bucket for insertion
    //         let mut next_empty_bucket = s;
    //         while self.bucket_info[next_empty_bucket as usize].is_occupied && next_empty_bucket < self.size as u32 - 1 {
    //             next_empty_bucket += 1;
    //         }
    //
    //         // // Check if there's an empty bucket for insertion
    //         // if next_empty_bucket == self.size as u32 - 1 && self.bucket_info[next_empty_bucket as usize].is_occupied {
    //         //     // Filter is full, handle accordingly (e.g., resize or return error)
    //         //     return; // Assuming you handle this case elsewhere
    //         // }
    //
    //         // Shift elements to the right starting from the last occupied bucket
    //         let mut i = next_empty_bucket;
    //         while i > s {
    //             self.buckets[i as usize] = self.buckets[(i - 1) as usize];
    //             self.bucket_info[i as usize].is_shifted = true;
    //             self.bucket_info[i as usize].is_occupied = self.bucket_info[(i - 1) as usize].is_occupied;
    //             self.bucket_info[i as usize].is_continuation = self.bucket_info[(i - 1) as usize].is_continuation;
    //             i -= 1;
    //         }
    //
    //         // Insert the fingerprint into the appropriate bucket
    //         self.buckets[s as usize] = r;
    //         self.bucket_info[s as usize].is_occupied = true;
    //         self.bucket_info[s as usize].is_shifted = true;
    //         self.bucket_info[s as usize].is_continuation = false;
    //         if !self.bucket_info[(s + 1) as usize].is_continuation && self.bucket_info[(s + 1) as usize].is_continuation {
    //             self.bucket_info[(s + 1) as usize].is_continuation = true;
    //         }
    //     }
    //     println!("{:?}, {}, {}", self.buckets, s, r);
    // }


    //TODO if index out of bounds, return false. this is not a circular array. otherwise return true.
    pub(crate) fn insert(&mut self, x: u64) -> bool {
        let fingerprint = self.fingerprint(x);
        let quotient = self.get_quotient(fingerprint);
        let remainder = self.get_remainder(fingerprint);

        let mut insertion_point = quotient;

        // Find the insertion point, handling continuations and shifted buckets.
        while self.bucket_info[insertion_point as usize].is_occupied {
            if !self.bucket_info[insertion_point as usize].is_shifted &&
                !self.bucket_info[insertion_point as usize].is_continuation {
                // If the current bucket is occupied and not shifted or a continuation,
                // move to the next bucket and continue.
                insertion_point += 1;
            } else if self.bucket_info[insertion_point as usize].is_continuation {
                // Follow the continuation until the end.
                while self.bucket_info[insertion_point as usize].is_continuation {
                    insertion_point += 1;
                }
                break;
            } else {
                // If the bucket is shifted but not a continuation, insert at this location.
                break;
            }

            if !(insertion_point < (self.size - 1) as u32) {
                return false;
            }

        }

        // If the bucket isn't occupied and isn't a continuation, insert the fingerprint here.
        if !self.bucket_info[insertion_point as usize].is_occupied &&
            !self.bucket_info[insertion_point as usize].is_continuation &&
            !self.bucket_info[insertion_point as usize].is_shifted {
            self.buckets[insertion_point as usize] = remainder;
            self.bucket_info[insertion_point as usize].is_occupied = true;
            self.bucket_info[insertion_point as usize].is_shifted = false;
            self.bucket_info[insertion_point as usize].is_continuation = false;
            return true;
        } else {
            // Otherwise, shift elements to the right and insert the fingerprint.
            let mut i = insertion_point;
            let mut last_occupied_bucket = insertion_point.saturating_sub(1);
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
                    self.bucket_info[i as usize].is_occupied = self.bucket_info[(i + 1) as usize].is_occupied;
                    self.bucket_info[i as usize].is_continuation = self.bucket_info[(i + 1) as usize].is_continuation;
                }
                i += 1;
            }

            // Insert the fingerprint into the appropriate bucket.
            self.buckets[last_occupied_bucket as usize] = remainder;
            self.bucket_info[last_occupied_bucket as usize].is_occupied = true;
            self.bucket_info[last_occupied_bucket as usize].is_shifted = true;
            self.bucket_info[last_occupied_bucket as usize].is_continuation = true;
            return true;
        }
        println!("{:?}, {}, {}", self.buckets, quotient, remainder);
    }

    // pub(crate) fn member(&self, x: u64) -> bool {
    //     let f = self.fingerprint(x);
    //     let q = self.get_quotient(f);
    //     let r = self.get_remainder(f);
    //
    //     if !self.bucket_info[q as usize].is_occupied {
    //         // No fingerprints in the filter that map to this bucket.
    //         return false;
    //     }
    //     else if self.buckets[q as usize] == r {
    //         println!("found");
    //         return true;
    //     }
    //
    //     // Walk back to find the beginning of the cluster
    //     let mut b = q;
    //     while self.bucket_info[b as usize].is_shifted && b > 0 {
    //         b -= 1;
    //     }
    //     // println!("e1");
    //
    //     // Walk forward to find the actual start of the run
    //     let mut s = b;
    //     let mut i = 0;
    //     while b != q {
    //         if i > 1000 {
    //             break;
    //         }
    //         i += 1;
    //         // Skip all elements in the current run
    //         while (s as u64) < self.size - 1 && self.bucket_info[s as usize].is_continuation {
    //             s += 1;
    //         }
    //
    //         // Find the next occupied bucket
    //         while (b as u64) < self.size - 1 && !self.bucket_info[b as usize].is_occupied {
    //             b += 1;
    //         }
    //     }
    //     // println!("e4");
    //
    //
    //     // Search for fr within the run
    //     while (s as u64) < self.size - 1 && !self.bucket_info[s as usize].is_continuation {
    //         if self.buckets[s as usize] == r {
    //             return true;
    //         }
    //         s += 1;
    //     }
    //     // println!("e5");
    //
    //
    //     // It's not in the filter
    //     // println!("{}", r);
    //     false
    // }

    pub(crate) fn member(&self, x: u64) -> bool {
        let f = self.fingerprint(x);
        let q = self.get_quotient(f);
        let r = self.get_remainder(f);

        if !self.bucket_info[q as usize].is_occupied {
            // there are no fingerprints in the filter that map to this bucket.
            return false;
        }
        else if self.buckets[q as usize] == r {
            // println!("{}, {}", self.buckets[q as usize], r);
            return true;
        }
        else if self.buckets[(q + 1) as usize] == r {
            // println!("{}, {}", self.buckets[q as usize], r);
            return true;
        }
        // println!("{}, {}", self.buckets[q as usize], r);


        let mut b = q;
        while self.bucket_info[b as usize].is_shifted && b > 0 {
            b = b - 1;
        }

        let mut s = b;
        let mut i = 0;
        while b != q {
            if i > 10000 {
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
        while !self.bucket_info[s as usize].is_continuation && (s as u64) < self.size - 1 {
            // println!("{}, {}", self.buckets[s as usize], r);

            if self.buckets[s as usize] == r {
                return true;
            }
            s = s + 1;
        }
        if self.buckets[s as usize] == r {
            return true;
        }
        // it's not in the filter.
        return false;
    }

    fn fingerprint(&self, x: u64) -> u32 {
        return utils::hash(x,32,self.hashes[0].0, self.hashes[0].1, self.hashes[0].2);
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