use rand::Rng;
use std::collections::HashSet;
use log::log;

#[path = "utils.rs"]
mod utils;
use utils::hash;
use utils::log_base;

pub struct BinaryFuseFilter {
    fingerprints: Vec<u32>,
    hashes: Vec<(u64,u64,u64)>,
    sigma: Vec<(u64, usize)>,
    size: u64,
    segment_length: u32,
    l: u32
}
impl BinaryFuseFilter {
    pub fn new(keys: Vec<u64>) -> BinaryFuseFilter {
        let mut filter = BinaryFuseFilter {
            fingerprints: vec![],
            hashes: vec![],
            sigma: vec![],
            size: 0,
            segment_length: 0,
            l: 0,
        };
        let n = keys.len();
        filter.size = ((1.23 * n as f64).floor() + 32.0) as u64;
        filter.l = 64 - (filter.size - 1).leading_zeros();
        let exp = (log_base(n as f64,3.33) + 2.25).floor() as u32;
        filter.segment_length = 2u32.pow(exp);
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
                let a1: u64 = rng.gen_range(1..=u64::MAX );
                let a2: u64 = rng.gen_range(1..=u64::MAX);
                let b: u64 = rng.gen_range(1..=u64::MAX);
                hash_functions.push((a1,a2,b));
            }
            self.hashes = hash_functions;
            if self.mapping(keys.clone()) {
                finished = true;
                //println!("mapping succeeded!");
                self.assign(self.sigma.clone());
            }
        }
    }
    fn mapping(&mut self, keys: Vec<u64>) -> bool {
        let c: u64 = self.size;
        let mut h: Vec<HashSet<u64>> = vec![ HashSet::new(); c as usize];
        for i in 0..keys.len() {
            let x = keys[i];
            let (h0,h1,h2) = self.segmented_hash(x);
            h[h0 as usize].insert(x);
            h[h1 as usize].insert(x);
            h[h2 as usize].insert(x);
        }
        let mut q = Vec::new();
        let mut sigma = Vec::new();
        for i in 0..h.len() {
            if h[i].len() == 1 {
                q.push(i);
            }
        }
        while !q.is_empty() {
            let i = q.remove(0);
            if h[i].len() == 1 {
                let mut x = 0;
                for elem in h[i].clone() {
                    x = elem;
                }
                // needs to be a stack.
                sigma.push((x, i));
                let (h0,h1,h2) = self.segmented_hash(x);
                // remove x from h[h_j]
                h[h0 as usize].remove(&x);
                h[h1 as usize].remove(&x);
                h[h2 as usize].remove(&x);
                if h[h0 as usize].len() == 1 {
                    q.push(h0 as usize);
                }
                if h[h1 as usize].len() == 1 {
                    q.push(h1 as usize);
                }
                if h[h2 as usize].len() == 1 {
                    q.push(h2 as usize);
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
    fn assign(&mut self, sigma: Vec<(u64, usize)>) {
        let c: u64 = self.size;
        let mut b = vec![0; c as usize ];
        for j in (0..sigma.len()).rev() {
            let (x,i) = sigma[j];
            let (h0,h1,h2) = self.segmented_hash(x);
            b[i] = self.fingerprint(x) ^ (b[h0 as usize] ^ b[h1 as usize] ^ b[h2 as usize]);
        }
        self.fingerprints = b;

        // clear sigma. it is no longer needed
        self.sigma = Vec::new();
    }

    pub(crate) fn fingerprint(&self, key: u64) -> u32 {
        return hash(key,self.l, self.hashes[0].0, self.hashes[0].1, self.hashes[0].2) as u32;
    }

    fn segmented_hash(&self, key: u64) -> (u32, u32, u32) {
        // select segment.
        // hash in segment range. using the three hash functions.
        let s_length = self.segment_length;
        let num_segments = self.size / self.segment_length as u64;
        let segment_id = hash(key,self.l, self.hashes[3].0, self.hashes[3].1, self.hashes[3].2) % num_segments as u32;

        let h0 = (hash(key,self.l, self.hashes[0].0, self.hashes[0].1, self.hashes[0].2) % s_length) + (segment_id * s_length);
        let h1 = (hash(key,self.l, self.hashes[1].0, self.hashes[1].1, self.hashes[1].2) % s_length) + (segment_id * s_length);
        let h2 = (hash(key,self.l, self.hashes[2].0, self.hashes[2].1, self.hashes[2].2) % s_length) + (segment_id * s_length);

        return (h0,h1,h2);
    }
}