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
        let length: usize = (1.0f64 * size as f64) as usize;
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


    pub(crate) fn insert(&mut self, x: u64) {
        let f = self.fingerprint(x);
        let q = self.get_quotient(f);
        let r = self.get_remainder(f);

        let mut s = q;

        // Find the insertion point. Continue until end of previous run.
        while self.bucket_info[s as usize].is_continuation {
            s += 1;
        }

        // Check new insertion point.
        if !self.bucket_info[s as usize].is_occupied && !self.bucket_info[s as usize].is_shifted &&
            !self.bucket_info[s as usize].is_continuation {
            // If the bucket is not occupied, insert the fingerprint
            self.buckets[s as usize] = r;
            self.bucket_info[s as usize].is_occupied = true;
            self.bucket_info[s as usize].is_shifted = false;
            self.bucket_info[s as usize].is_continuation = false;
        }
        else {
            // the bucket is occupied - find the next available bucket
            while self.bucket_info[s as usize].is_occupied && s < self.size as u32 - 1 {
                s += 1;
            }

            // Right shift all elements after the insertion point
            let mut i = s;
            let mut last_occupied_bucket = s - 1;
            while i > q {
                if self.bucket_info[i as usize].is_occupied {
                    last_occupied_bucket = i;
                }
                i -= 1;
            }

            // Shift elements to the right
            while i > q {
                if self.bucket_info[i as usize].is_occupied {
                    // If the bucket is occupied, shift its content to the next bucket
                    self.buckets[i as usize] = self.buckets[(i - 1) as usize];
                    self.bucket_info[i as usize].is_shifted = true;
                    self.bucket_info[i as usize].is_occupied = self.bucket_info[(i - 1) as usize].is_occupied;
                    self.bucket_info[i as usize].is_continuation = self.bucket_info[(i - 1) as usize].is_continuation;
                }
                i -= 1;
            }
            // Insert the fingerprint into the appropriate bucket
            self.buckets[s as usize] = r;
            self.bucket_info[s as usize].is_occupied = true;
            self.bucket_info[s as usize].is_shifted = true;
            self.bucket_info[s as usize].is_continuation = false;
        }
    }

    pub(crate) fn member(&self, x: u64) -> bool {
        let f = self.fingerprint(x);
        let q = self.get_quotient(f);
        let r = self.get_remainder(f);

        if !self.bucket_info[q as usize].is_occupied {
            // there are no fingerprints in the filter that map to this bucket.
            return false;
        }

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
            if self.buckets[s as usize] == r {
                return true;
            }
            s = s + 1;
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