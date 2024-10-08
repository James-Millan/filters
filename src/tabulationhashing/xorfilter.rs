use rand::Rng;
use std::collections::{HashSet, VecDeque};
#[path = "utils.rs"]
mod utils;
use utils::hash;

pub struct XorFilter {
    fingerprints: Vec<u32>,
    hashes: Vec<(u64,u64,u64)>,
    sigma: Vec<(u64, usize)>,
    size: u64,
    l: u32
}
impl XorFilter {
    pub fn new(keys: Vec<u64>) -> XorFilter {
        let mut filter = XorFilter {
            fingerprints: vec![],
            hashes: vec![],
            sigma: vec![],
            size: 0,
            l: 0,
        };

        filter.size = ((1.23 * keys.len() as f64).floor() + 32.0) as u64;
        // if (filter.size < 1000) {
        //     filter.size = 10000;
        // }
        filter.l = utils::log_base(filter.size as f64, 2f64) as u32;
        filter.construct(keys);
        return filter;
    }
    pub fn member(&self, k: u64) -> bool {
        let h0 = self.hash0(k);
        let h1 = self.hash1(k);
        let h2 = self.hash2(k);
        let f = self.fingerprint(k);
        return f == (self.fingerprints[h0 as usize] ^ self.fingerprints[h1 as usize] ^ self.fingerprints[h2 as usize]);
    }
    fn construct(&mut self, keys: Vec<u64>) {
        let mut finished = false;
        while !finished {
            let mut rng = rand::thread_rng();
            let mut hash_functions = Vec::new();

            for _ in 0..=2 {
                let a1: u64 = rng.gen_range(1..=u64::MAX );
                let a2: u64 = rng.gen_range(1..=u64::MAX);
                let b: u64 = rng.gen_range(1..=u64::MAX);
                hash_functions.push((a1,a2,b));
            }
            self.hashes = hash_functions;
            if self.mapping(&keys) {
                finished = true;
                //println!("mapping succeeded!");
                self.assign();
            }
        }
    }
    fn mapping(&mut self, keys: &Vec<u64>) -> bool {
        let c: u64 = self.size;
        let mut h: Vec<(u64,usize)> = vec![(0,0); c as usize];
        for i in 0..keys.len() {
            let x = keys[i];
            let h0 = self.hash0(x);
            let h1 = self.hash1(x);
            let h2 = self.hash2(x);
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
                let h0 = self.hash0(x);
                let h1 = self.hash1(x);
                let h2 = self.hash2(x);
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
            //println!("'{:?}'", sigma);
            //println!("'{}'", sigma.len());
            return false;
        }
    }
    fn assign(&mut self) {
        let c: u64 = self.size;
        let mut b = vec![0; c as usize ];
        for j in (0..self.sigma.len()).rev() {
            let (x,i) = self.sigma[j];
            b[i] = self.fingerprint(x) ^ (b[self.hash0(x) as usize] ^ b[self.hash1(x) as usize] ^ b[self.hash2(x) as usize]);
        }
        self.fingerprints = b;

        // clear sigma. it is no longer needed
        self.sigma = Vec::new();
    }

    pub(crate) fn fingerprint(&self, key: u64) -> u32 {
        return hash(key,self.l, self.hashes[0].0, self.hashes[0].1, self.hashes[0].2) as u32;
    }
    fn hash0(&self, key: u64) -> u32 {
        let bound = self.size / 3;
        let res = hash(key,self.l, self.hashes[0].0, self.hashes[0].1, self.hashes[0].2) % bound as u32;
        //println!("'{}','{}'",0, res);
        return res;

    }
    fn hash1(&self, key: u64) -> u32 {
        let bound = self.size / 3;
        let mut res = hash(key,self.l, self.hashes[1].0, self.hashes[1].1, self.hashes[1].2) % bound as u32;
        res = (bound + res as u64) as u32;
        //println!("'{}','{}'",1, res);
        return res
    }
    fn hash2(&self, key: u64) -> u32 {
        let bound = self.size / 3;
        let mut res = (hash(key, self.l, self.hashes[2].0, self.hashes[2].1, self.hashes[2].2)) % bound as u32;
        res = ((2 * bound) + res as u64) as u32;
        //println!("'{}','{}'",2, res);
        return res;
    }
}