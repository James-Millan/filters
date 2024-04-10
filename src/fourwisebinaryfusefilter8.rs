use rand::Rng;
use std::collections::{HashSet, VecDeque};


#[path = "utils.rs"]
mod utils;
use utils::hash;
use utils::log_base;

pub struct FourWiseBinaryFuseFilter8 {
    fingerprints: Vec<u8>,
    hashes: Vec<(u64,u64,u64)>,
    sigma: Vec<(u64, usize)>,
    size: u64,
    segment_length: u32,
    l: u32,
    log_segment: u32,
    num_segments: u64
}
impl FourWiseBinaryFuseFilter8 {
    pub fn new(keys: &Vec<u64>) -> FourWiseBinaryFuseFilter8 {
        let mut filter = FourWiseBinaryFuseFilter8 {
            fingerprints: vec![],
            hashes: vec![],
            sigma: vec![],
            size: 0,
            segment_length: 0,
            l: 0,
            log_segment: 0,
            num_segments: 0
        };
        let n = keys.len();
        filter.size = ((1.075 * n as f64).floor() + 32.0) as u64;
        filter.l = log_base(filter.size as f64, 2f64) as u32;
        let exp = (log_base(n as f64,2.91) -0.5).floor() as u32;
        filter.segment_length = 2u32.pow(exp);
        filter.log_segment = log_base(filter.segment_length as f64, 2f64) as u32;
        filter.num_segments = filter.size / filter.segment_length as u64;
        if (filter.num_segments < 4 ) {
            filter.size = (filter.segment_length * 4) as u64;
            filter.num_segments = 4;
            filter.l =  log_base(filter.size as f64, 2f64) as u32;
        }
        filter.construct(keys);
        return filter;
    }
    pub fn member(&self, k: u64) -> bool {
        let (h0,h1,h2,h3) = self.segmented_hash(k);
        let f = self.fingerprint(k);
        return f == (self.fingerprints[h0 as usize] ^ self.fingerprints[h1 as usize] ^ self.fingerprints[h2 as usize] ^
        self.fingerprints[h3 as usize]);
    }
    fn construct(&mut self, keys: &Vec<u64>) {
        let mut finished = false;
        while !finished {
            let mut rng = rand::thread_rng();
            let mut hash_functions = Vec::new();

            for _ in 0..=4 {
                let a1: u64 = rng.gen_range(1..=u64::MAX );
                let a2: u64 = rng.gen_range(1..=u64::MAX);
                let b: u64 = rng.gen_range(1..=u64::MAX);
                hash_functions.push((a1,a2,b));
            }
            self.hashes = hash_functions;
            if self.mapping(keys) {
                finished = true;
                // println!("mapping succeeded!");
                self.assign();
            }
            else {
                // println!("mapping failed!");
                // println!("{} {} {}", self.l, self.segment_length, self.log_segment)
            }
        }
    }
    fn mapping(&mut self, keys: &Vec<u64>) -> bool {
        let c: u64 = self.size;
        let mut h: Vec<(u64,usize)> = vec![(0,0); c as usize];
        for i in 0..keys.len() {
            let x = keys[i];
            let (h0,h1,h2,h3) = self.segmented_hash(x);
            h[h0 as usize] = ((x ^ h[h0 as usize].0), h[h0 as usize].1 + 1);
            h[h1 as usize] = ((x ^ h[h1 as usize].0), h[h1 as usize].1 + 1);
            h[h2 as usize] = ((x ^ h[h2 as usize].0), h[h2 as usize].1 + 1);
            h[h3 as usize] = ((x ^ h[h3 as usize].0), h[h3 as usize].1 + 1);


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
                let (h0,h1,h2,h3) = self.segmented_hash(x);
                // remove x from h[h_j]
                h[h0 as usize] = ((x ^ h[h0 as usize].0), h[h0 as usize].1 - 1);
                h[h1 as usize] = ((x ^ h[h1 as usize].0), h[h1 as usize].1 - 1);
                h[h2 as usize] = ((x ^ h[h2 as usize].0), h[h2 as usize].1 - 1);
                h[h3 as usize] = ((x ^ h[h3 as usize].0), h[h3 as usize].1 - 1);

                if h[h0 as usize].1 == 1 {
                    q.push_back(h0 as usize);
                }
                if h[h1 as usize].1 == 1 {
                    q.push_back(h1 as usize);
                }
                if h[h2 as usize].1 == 1 {
                    q.push_back(h2 as usize);
                }
                if h[h3 as usize].1 == 1 {
                    q.push_back(h3 as usize);
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
            //println!("'{}'", sigma.len());
            return false;
        }
    }
    fn assign(&mut self) {
        let c: u64 = self.size;
        let mut b = vec![0; c as usize ];
        for j in (0..self.sigma.len()).rev() {
            let (x,i) = self.sigma[j];
            let (h0,h1,h2,h3) = self.segmented_hash(x);
            b[i] = self.fingerprint(x) ^ (b[h0 as usize] ^ b[h1 as usize] ^ b[h2 as usize] ^ b[h3 as usize]);
        }
        self.fingerprints = b;

        // clear sigma. it is no longer needed
        self.sigma = Vec::new();
    }

    pub(crate) fn fingerprint(&self, key: u64) -> u8 {
        return hash(key,self.l, self.hashes[0].0, self.hashes[0].1, self.hashes[0].2) as u8;
    }

    fn segmented_hash(&self, key: u64) -> (u32, u32, u32,u32) {
        // select segment.
        // hash in segment range. using the three hash functions.
        let s_length = self.segment_length;
        // hash this to the correct range. then complete.
        let segment_id = hash(key,self.l, self.hashes[4].0, self.hashes[4].1, self.hashes[4].2)
            % (self.num_segments-3) as u32;
        let h0 = (hash(key,self.log_segment, self.hashes[0].0, self.hashes[0].1, self.hashes[0].2) ) +
            (segment_id * s_length);
        let h1 = (hash(key,self.log_segment, self.hashes[1].0, self.hashes[1].1, self.hashes[1].2)) +
            ((segment_id+1) * s_length);
        let h2 = (hash(key,self.log_segment, self.hashes[2].0, self.hashes[2].1, self.hashes[2].2)) +
            ((segment_id+2) * s_length);

        let h3 = (hash(key,self.l, self.hashes[3].0, self.hashes[3].1, self.hashes[3].2) % s_length) +
            ((segment_id+3) * s_length);
        return (h0,h1,h2,h3);
    }
}