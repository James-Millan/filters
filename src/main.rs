mod bloomfilter;
mod cuckoofilter;
mod countingbloomfilter;
mod xorfilter;
mod bitvector;
mod blockedbloomfilter;
mod utils;
mod dleftcountingfilter;
mod registeralignedbloomfilter;
mod threewisebinaryfusefilter;

extern crate rand;

use std::cell::RefCell;
use rand::Rng;
use crate::cuckoofilter::CuckooFilter;
use crate::utils::hash;
use crate::xorfilter::XorFilter;

fn main() {
    let mut keys = Vec::new();

    for i in 0..=1000000 {
        //list.push(XorFilter::fingerprint(i));
        keys.push(i);
    }

    let mut binaryfusefilter = threewisebinaryfusefilter::BinaryFuseFilter::new(keys);

    for j in 0..=100000 {
        println!("Contains '{}': {}", j, binaryfusefilter.member(j));
    }
}

static size:u64 = ((1.23 * 100f32) + 32.0) as u64;
static l:u32 = 64 - (size - 1).leading_zeros();

fn get_hashes() -> Vec<(u64, u64, u64)> {
    let mut rng = rand::thread_rng();
    let mut hash_functions = Vec::new();

    for _ in 0..=2 {
        let a1: u64 = rng.gen_range(1..=u64::MAX );
        let a2: u64 = rng.gen_range(1..=u64::MAX);
        let b: u64 = rng.gen_range(1..=u64::MAX);
        hash_functions.push((a1,a2,b));
    }
    return hash_functions;
}
fn hash0(key: u64, hashes: Vec<(u64,u64,u64)>) -> u32 {
    let bound = size / 3;
    let res = hash(key,l, hashes[0].0, hashes[0].1, hashes[0].2) % bound as u32;
    //println!("'{}','{}'",0, res);
    return res;

}

fn hash1(key: u64, hashes: Vec<(u64,u64,u64)>) -> u32 {
    let bound = size / 3;

    let mut res = hash(key,l, hashes[1].0, hashes[1].1, hashes[1].2) % bound as u32;
    res = (bound + res as u64) as u32;
    //println!("'{}','{}'",1, res);
    return res
}

fn hash2(key: u64, hashes: Vec<(u64,u64,u64)>) -> u32 {
    let bound = size / 3;
    let mut res = (hash(key, l, hashes[2].0, hashes[2].1, hashes[2].2)) % bound as u32;
    res = ((2 * bound) + res as u64) as u32;
    //println!("'{}','{}'",2, res);
    return res;
}