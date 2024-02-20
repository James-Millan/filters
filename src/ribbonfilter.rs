use std::collections::hash_map::Keys;
use rand::Rng;
use slab::Slab;
use crate::utils;
use crate::utils::{hash, log_base};

// fn init() {
//     let ribbon_width = 0; // this is w
//     let starting_position = 0; // this is s. \in [m-w-1] random
//     let coefficient_vector =0; // this is c. {0,1}^w. \\ force it to start with 1.
//     let h = 0; // this is 0^s-1 c(x) 0^{m-s-w+1}
//     // matrix with rows h(x), sorted by s(x)
//     // has all of its 1-entries in a “ribbon” of width 𝑤 that randomly passes through
//     // the matrix from the top left to the bottom right,
//     // a solution Z to h(x) . Z = b(x) can be computed quickly.
// }


// have a vec of ribbon entries. these will consist of a start position and a coefficient vector. Trailing zeroes easily
// calculable from this. Each index in the vec corresponds to a row in the matrix. Columns are different but we don't
// really calculate based off of column anyway.

pub struct RibbonEntry {
    start_pos: u64,
    coefficient_vector: u64,
}


pub struct RibbonFilter {
    size: u64,
    l: u32,
    epsilon: f64,
    matrix: Vec<RibbonEntry>,
    keys: Vec<u64>,
    c: Vec<u64>,
    b: Vec<u64>,
    ribbon_width: usize,
    hash_functions: Vec<(u64,u64,u64)>,
    binary_info: (u32,u32)
}

impl RibbonFilter {
    pub(crate) fn new(keys: &Vec<u64>, epsilon: f64) -> RibbonFilter {
        let size = ((1f64-epsilon) * (keys.len() as f64)) as u64;
        let cloned_keys = keys.clone();
        let mut filter = RibbonFilter {
            size,
            l: utils::log_base(size as f64, 2f64) as u32,
            epsilon,
            matrix: vec![],
            keys: cloned_keys,
            c: vec![0; size as usize],
            b: vec![0; size as usize],
            ribbon_width: 16,
            hash_functions: vec![],
            binary_info: (0, 0),
        };
        filter.hash_functions = Self::generate_hash_functions(size as usize);
        filter.generate_matrix();
        return filter;

    }

    fn generate_matrix(&mut self) {
        let n = (self.size as f64 / (1f64-self.epsilon)) as u64;
        for key in &self.keys {
            let mut start_pos = self.starting_position(*key);
            let mut coef_vec = self.coefficient_vector(*key);
            // let trailing = (self.size as i64 - (self.ribbon_width as i64) - (start_pos as i64) + 1i64) as u64;
            println!("{} {} {}", self.size, self.ribbon_width, start_pos);
            let trailing = ((self.size as i128).saturating_sub(self.ribbon_width as i128)).saturating_sub(start_pos as i128).saturating_add(1) as u128;

            let bitstring = Self::generate_bitstring(start_pos, coef_vec as u64,trailing as u32);
            let mut b = self.fingerprint(*key);
            loop {
                if self.c[start_pos as usize] == 0 {
                    self.c[start_pos as usize] = coef_vec as u64;
                    self.b[start_pos as usize] = b;
                    break;
                }
                coef_vec = coef_vec ^ self.c[start_pos as usize] as u32;
                b = b ^ self.b[start_pos as usize];
                if coef_vec == 0 {
                    if b == 0 {
                        break;
                    }
                    else {
                        //TODO ERROR HAS HAPPENED
                        break;
                    }
                }
                let j =  coef_vec & (1 << coef_vec.trailing_zeros());
                start_pos = start_pos + j;
                coef_vec = coef_vec >> j;
            }
        }
    }

    pub(crate) fn member(&self, x: u64) -> bool {
        return false;
    }

    pub(crate) fn fingerprint(&self, key: u64) -> u64 {
        return utils::hash(key,self.l, self.hash_functions[2].0, self.hash_functions[2].1, self.hash_functions[2].2) as u64;
    }

    fn generate_hash_functions(n: usize) -> Vec<(u64, u64,u64)> {
        let mut rng = rand::thread_rng();
        let mut hash_functions = Vec::new();

        for _ in 0..3 {
            let a1: u64 = rng.gen_range(1..=u64::MAX );
            let a2: u64 = rng.gen_range(1..=u64::MAX);
            let b: u64 = rng.gen_range(1..=u64::MAX);
            hash_functions.push((a1,a2,b));
        }
        return hash_functions;
    }

    fn generate_bitstring(leading_zeroes: u32, bitstring: u64, trailing_zeroes: u32) -> u64 {
        // let trailing_zeroes = (1 >> trailing_zeroes);
        println!("NUMBER OF TRAILING ZEROES IS: '{}'", trailing_zeroes);
        return (bitstring << trailing_zeroes);
    }

    fn starting_position(&self, x:u64) -> u32 {
        let range = self.size - self.ribbon_width as u64 - 1;
        let l = log_base(log_base(range as f64, 2f64).ceil(), 2f64) as u32;
        // println!("'{}'", l);
        return utils::hash(x, l as u32, self.hash_functions[0].0,self.hash_functions[0].1,self.hash_functions[0].2);
    }

    fn coefficient_vector(&self, x:u64) -> u32 {
        let res = utils::hash(x, self.ribbon_width as u32, self.hash_functions[1].0,self.hash_functions[1].1,self.hash_functions[1].2);
        // println!("'{}'", res);
        return res;
    }
}

