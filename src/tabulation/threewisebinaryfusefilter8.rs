use std::cmp;
use rand::Rng;
use std::collections::{HashSet, VecDeque};
#[path = "tabulationhashing.rs"]
mod tabulationhashing;
use tabulationhashing::TabulationHashing;

#[path = "../utils.rs"]
mod utils;
use utils::log_base;

pub struct ThreeWiseBinaryFuseFilter8 {
    fingerprints: Vec<u8>,
    hashes: Vec<TabulationHashing>,
    sigma: Vec<(u64, usize)>,
    size: u64,
    segment_length: u32,
    num_segments: u64,
}
impl ThreeWiseBinaryFuseFilter8 {
    pub fn new(keys: Vec<u64>) -> ThreeWiseBinaryFuseFilter8 {
        let mut filter = ThreeWiseBinaryFuseFilter8 {
            fingerprints: vec![],
            hashes: vec![],
            sigma: vec![],
            size: 0,
            segment_length: 0,
            num_segments: 0,
        };
        let n = keys.len();
        filter.size = (1.23 * n as f64) as u64 + 64;
        // filter.size =  ((0.875 + 0.25 * cmp::max(1, (log_base(100000f64, 2f64) / log_base(n as f64, 2f64)) as i32)
        //     as f64) as usize * n) as u64;
            //(1.125 * n as f64).floor() + 32.0) as u64;
        let exp = (log_base(n as f64,3.33) + 2.25).floor() as u32;
        filter.segment_length = 2u32.pow(exp);
        filter.num_segments = filter.size / filter.segment_length as u64;
        if (filter.num_segments < 3 ) {
            filter.size = (filter.segment_length * 3) as u64;
            filter.num_segments = 3;
        }
        filter.construct(keys);
        return filter;
    }
    pub fn member(&self, k: u64) -> bool {
        let (h0,h1,h2) = self.segmented_hash(k);
        let f = self.fingerprint(k);
        return f == (self.fingerprints[h0 as usize] ^ self.fingerprints[h1 as usize] ^ self.fingerprints[h2 as usize]);
    }
    fn construct(&mut self, keys: Vec<u64>) {
        let mut finished = false;
        while !finished {
            let mut rng = rand::thread_rng();
            let mut hash_functions = Vec::new();

            for _ in 0..=3 {
                hash_functions.push(TabulationHashing::new());
            }
            self.hashes = hash_functions;
            if self.mapping(&keys) {
                finished = true;
                println!("mapping succeeded!");
                self.assign();
            }
            else {
                //println!("mapping failed!");
            }
        }
    }
    fn mapping(&mut self, keys: &Vec<u64>) -> bool {
        let c: u64 = self.size;
        let mut h: Vec<(u64,usize)> = vec![(0,0); c as usize];
        for i in 0..keys.len() {
            let x = keys[i];
            let (h0,h1,h2) = self.segmented_hash(x);
            h[h0 as usize] = ((x ^ h[h0 as usize].0), h[h0 as usize].1 + 1);
            h[h1 as usize] = ((x ^ h[h1 as usize].0), h[h1 as usize].1 + 1);
            h[h2 as usize] = ((x ^ h[h2 as usize].0), h[h2 as usize].1 + 1);
        }
        let mut q = VecDeque::new();
        let mut sigma = Vec::new();
        for i in 0..h.len() {
            if h[i].1 == 1 {
                q.push_back(i);
            }
        }
        while !q.is_empty() {
            let i = q.pop_front().unwrap();
            if h[i].1 == 1 {
                let x = h[i].0;
                // needs to be a stack.
                sigma.push((x, i));
                let (h0,h1,h2) = self.segmented_hash(x);
                // remove x from h[h_j]
                h[h0 as usize] = ((x ^ h[h0 as usize].0), h[h0 as usize].1 - 1);
                h[h1 as usize] = ((x ^ h[h1 as usize].0), h[h1 as usize].1 - 1);
                h[h2 as usize] = ((x ^ h[h2 as usize].0), h[h2 as usize].1 - 1);
                if h[h0 as usize].1 == 1 {
                    q.push_back(h0 as usize);
                }
                if h[h1 as usize].1 == 1 {
                    q.push_back(h1 as usize);
                }
                if h[h2 as usize].1 == 1 {
                    q.push_back(h2 as usize);
                }
            }
        }
        if sigma.len() == keys.len() {
            //println!("'{}'", sigma.len());
            self.sigma = sigma;
            return true;
        }
        else {
            // println!("'{:?}'", sigma);
            // println!("'{}'", sigma.len());
            return false;
        }
    }
    fn assign(&mut self) {
        let c: u64 = self.size;
        let mut b = vec![0; c as usize ];
        for j in (0..self.sigma.len()).rev() {
            let (x,i) = self.sigma[j];
            let (h0,h1,h2) = self.segmented_hash(x);
            b[i] = self.fingerprint(x) ^ (b[h0 as usize] ^ b[h1 as usize] ^ b[h2 as usize]);
        }
        self.fingerprints = b;

        // clear sigma. it is no longer needed
        self.sigma = Vec::new();
    }

    pub(crate) fn fingerprint(&self, key: u64) -> u8 {
        return (self.hashes[3].tabulation_hashing(key) % self.size) as u8;
    }

    // select a segment via a hash function. hash three times in [segment_length]. return 3 indexes in 3 consecutive segments.
    fn segmented_hash(&self, key: u64) -> (u32, u32, u32) {
        // select segment.
        // hash in segment range. using the three hash functions.
        let s_length = self.segment_length;
        // hash this to the correct range. then complete.
        let segment_id = self.hashes[3].tabulation_hashing(key)
            % (self.num_segments-2);
        let h0: u32 = ((self.hashes[0].tabulation_hashing(key) % s_length as u64) + (segment_id * s_length as u64)) as u32;
        let h1: u32 = ((self.hashes[1].tabulation_hashing(key) % s_length as u64) + ((segment_id + 1) * s_length as u64)) as u32;
        let h2: u32 = ((self.hashes[2].tabulation_hashing(key) % s_length as u64) + ((segment_id + 2) * s_length as u64)) as u32;
        //println!("{:?}", (h0, h1, h2));
        return (h0,h1,h2);
    }
}