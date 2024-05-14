mod bloomfilter;
mod cuckoofilter;
mod countingbloomfilter;
mod xorfilter;
mod bitvector;
mod blockedbloomfilter;
mod utils;
mod registeralignedbloomfilter;
mod threewisebinaryfusefilter32;
mod fourwisebinaryfusefilter32;
mod threewisebinaryfusefilter16;
mod threewisebinaryfusefilter8;
mod fpr;
mod XorFilter8;
mod registeralignedlarger;
mod tabulationhashing;
mod keygenerator;
#[path = "tabulation/bloomfilter.rs"]
mod btab;
#[path = "tabulation/countingbloomfilter.rs"]
mod cbtab;
#[path = "tabulation/blockedbloomfilter.rs"]
mod bbtab;
#[path = "tabulation/registeralignedbloomfilter.rs"]
mod rabtab;
#[path = "tabulation/cuckoofilter.rs"]
mod ctab;
#[path = "tabulation/XorFilter8.rs"]
mod xtab;
#[path = "tabulation/threewisebinaryfusefilter8.rs"]
mod bftab;
#[path = "tabulation/fpr.rs"]
mod fprtab;

#[path = "fasthash/bloomfilter.rs"]
mod bffast;
#[path = "fasthash/countingbloomfilter.rs"]
mod cbffast;
#[path = "fasthash/blockedbloomfilter.rs"]
mod bbffast;
#[path = "fasthash/registeralignedbloomfilter.rs"]
mod rabbffast;
#[path = "fasthash/fpr.rs"]
mod fprfast;
mod mortonfilter;
mod MortonBlock;
mod quotientfilter;
mod quotientinfo;
mod fourwisebinaryfusefilter8;
mod fourwisebinaryfusefilter16;

extern crate rand;


use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use rand::Rng;
use rand::seq::index::sample;
use crate::rabtab::RegisterAlignedBloomFilter;


fn main() {
let mut mf = mortonfilter::MortonFilter::new(1000, 0.01);
    let mut succ = vec![];
    for i in 0..1000 {
        if mf.insert(i) {
            succ.push(i);
        }
    }
    // println!("{:?}", mf.block_store);
    for i in succ {
        println!("Contains: {},{}", i, mf.member(i));
    }
}
