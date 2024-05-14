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
    // let mut rng = rand::thread_rng();
    // let random_index = rng.gen_range(off..off+overflow_check);
    // let x = mf.hash1(10);
    // let f = mf.fingerprint(10);
    // let y = mf.hash2(x, f);
    // let z = mf.hash_prime(y as usize, f);
    // let w = mf.hash_prime(x as usize, f);
    // let v = mf.hash_prime(w as usize,f);
    // let o = mf.offset(f);
    println!("{}", mf.size);
    // println!("h1,h2,h2->h1, h1->h2, (h1->h2)->h1, offset(f)");
    // println!("{},{},{},{} {} {}", x, y, z, w, v, o);


    // for i in 0..100000 {
    //     let f = mf.fingerprint(i);
    //     let h1 = mf.hash1(i);
    //     let h2 = mf.hash2(h1,f);
    //     let h1th2 = mf.hash_prime(h1 as usize, f);
    //     let h2th1 = mf.hash_prime(h2 as usize, f);
    //     println!("{},{},{},{},{}",h1,h2,h1th2,h2th1,mf.offset(f));
    //
    //     assert_eq!(h1,h2th1);
    //     assert_eq!(h2, h1th2);
    // }
    let mut check: HashMap<u32,(u64,u8)> = HashMap::new();
    for i in 0..1000 {
        // let h1 = mf.hash1(i);
        // let f = mf.fingerprint(i);
        // if (check.contains_key(&h1)) {
        //     let ret = *check.get(&h1).unwrap();
        //     let retrieved = ret.1;
        //         println!("{:?}, {}, {}, {}, {}", check.get(&h1), h1, mf.hash2(h1, retrieved), mf.hash_prime(h1 as usize, retrieved), mf.offset(retrieved));
        // }
        // check.insert(mf.hash1(i),(i,mf.fingerprint(i)));

        if mf.insert(i) {
            succ.push(i);
        }
    }
    // println!("{:?}", mf.block_store);
    for i in succ {
        if !mf.member(i) {
            println!("Contains: {},{}", i, mf.member(i));
        }
    }
}
