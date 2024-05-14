use rand::Rng;

#[path = "MortonBlock.rs"]
mod mortonblock;
use mortonblock::MortonBlock;
#[path = "utils.rs"]
mod utils;
use utils::{hash};

const BUCKETS_PER_BLOCK: u32 = 46;
const OFF_RANGE: u32 = 256;
const FCA_MAX_VAL: u8 = 3;

pub struct MortonFilter {
    pub(crate) block_store: Vec<MortonBlock>,
    hashes: Vec<(u64,u64,u64)>,
    cache_size: usize,
    pub(crate) size: u64,
    l: u32,
}

impl MortonFilter {
    pub fn new(size : u64, fpr: f64) -> MortonFilter {
        let length = size as u64;
        let n = length * BUCKETS_PER_BLOCK as u64;
        return MortonFilter {
            block_store: Self::generate_block_store(length),
            hashes: Self::generate_hash_functions(4),
            cache_size: 512,
            size: n,
            l: utils::log_base(length as f64, 2f64) as u32
        }
    }

    fn generate_block_store(length : u64) -> Vec<MortonBlock> {
        let mut bs = Vec::new();
        for _ in 0..length {
            bs.push(MortonBlock::new());
        }
        return bs;
    }

    fn generate_hash_functions(n: usize) -> Vec<(u64, u64,u64)> {
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

    pub fn insert(&mut self, x: u64) -> bool {
        // obtain indices
        let f = self.fingerprint(x);
        let glbi1 = self.hash1(x);
        let mut block1: &mut MortonBlock = &mut self.block_store[(glbi1/BUCKETS_PER_BLOCK) as usize];
        let lbi1 = glbi1 % BUCKETS_PER_BLOCK;

        // calculate offset.
        let mut off = 0;
        for i in 0..lbi1 {
            off += block1.fca[i as usize];
        }
        //overflow condition
        // calculate FCA value. must be less than 3
        let overflow_check = block1.fca[lbi1 as usize];
        if off + overflow_check >= BUCKETS_PER_BLOCK as u8 || overflow_check >= FCA_MAX_VAL {
            // set OTA
            let index = utils::map(lbi1 as u64, 16);
            let mut ota = &mut block1.ota;
            ota[index as usize] = 1;
            let glbi2 = self.hash2(glbi1 + lbi1, f);
            let mut block2: &mut MortonBlock  = &mut self.block_store[(glbi2/BUCKETS_PER_BLOCK) as usize];
            let lbi2 = glbi2 % BUCKETS_PER_BLOCK;

            // calculate offset
            // perform lookup and see if exists.
            let mut off2 = 0;
            for i in 0..lbi2 {
                off2 += block2.fca[i as usize];
            }
            //check FCA isn't full
            let overflow_check = block2.fca[lbi2 as usize];
            if off2 + overflow_check >= BUCKETS_PER_BLOCK as u8 || overflow_check >= FCA_MAX_VAL {
                return false;
                // perform eviction from block 1
                let mut fingerprint = f;
                let mut global_index = glbi1;
                let mut block: &mut MortonBlock = &mut self.block_store[(glbi1/BUCKETS_PER_BLOCK) as usize];
                let mut local_index = global_index % BUCKETS_PER_BLOCK;

                // calculate offset.
                let mut off = 0;
                for i in 0..local_index {
                    off += block.fca[i as usize];
                }
                let mut num_kicks = 0;
                while num_kicks < 1000 {
                    num_kicks += 1;
                    let overflow_check = block.fca[local_index as usize];
                    let mut rng = rand::thread_rng();
                    let random_index = rng.gen_range(off..=off+overflow_check);
                    let kicked_key = block.fsa[random_index as usize];
                    block.fsa[random_index as usize] = fingerprint;

                    // map evicted key to alternate bucket
                    let alternate = self.hash_prime(global_index as usize, kicked_key);
                    let mut alternate_block: &mut MortonBlock = &mut self.block_store[(alternate/BUCKETS_PER_BLOCK) as usize];
                    let mut alternate_local = alternate % BUCKETS_PER_BLOCK;
                    let mut offa = 0;
                    for i in 0..alternate_local {
                        offa += alternate_block.fca[i as usize];
                    }

                    let overflow_check = alternate_block.fca[local_index as usize];
                    if offa + overflow_check >= BUCKETS_PER_BLOCK as u8 || overflow_check >= FCA_MAX_VAL {
                        println!("need to evict again");
                        // another round set variables appropriately
                        fingerprint = kicked_key;
                        global_index = alternate;
                        block = alternate_block;
                        local_index = alternate_local;
                        off = offa;
                    }
                    else {
                        // insert key and move on
                        println!("done evicting");
                        alternate_block.fsa[(offa + overflow_check) as usize] = kicked_key;
                        alternate_block.fca[alternate_local as usize] += 1;
                        return true;
                    }

                }

                return false;
            }
            else {
                // insert into fsa
                let mut fsa: &mut Vec<u8> = &mut block2.fsa;
                fsa.insert((off2 + overflow_check) as usize, f);
                block2.fca[lbi2 as usize] += 1;
                return true;
            }
        }
        else {
            // insert into fsa
            let mut fsa= &mut block1.fsa;
            fsa.insert((off+overflow_check) as usize, f);
            block1.fca[lbi1 as usize] += 1;
            return true;
        }
        return false;
    }












    /*

                // println!("eviction needed, {}", x);
                // return false;
                // initialise mutable variables to be used on each iteration. evict from bucket1.
                let mut num_kicks = 0;
                let mut finished = false;
                let mut offset = off;
                let mut block: &mut MortonBlock = &mut self.block_store[(glbi1/BUCKETS_PER_BLOCK) as usize];
                let mut f = f;
                let mut local_index = lbi1;
                let mut remapped_index = glbi1;
                // assert that we are retrieving the correct h1 for the evicted key.

                while !finished && num_kicks < 1000 {
                    num_kicks += 1;
                    // select a key to evict and update fsa with new key.
                    let mut fsa: &mut Vec<u8> = &mut block.fsa;
                    let evicted_key = fsa[offset as usize];
                    // println!("{}", evicted_key);
                    //println!("eviction needed, {}, {}", x, evicted_key);

                    fsa[offset as usize] = f;
                    f = evicted_key;

                    // update ota to mark eviction
                    let index = utils::map(local_index as u64, 16);
                    let mut ota = &mut block.ota;
                    ota[index as usize] = 1;

                    //remap evicted key using hash_prime
                    // let r = self.hash_prime(remapped_index as usize, evicted_key);
                    // let rr = self.hash_prime(r as usize, evicted_key);
                    // let rrr = self.hash_prime(rr as usize, evicted_key);
                    //
                    // println!("{},{},{},{}",remapped_index,r,rr,rrr);
                    let r = remapped_index;
                    remapped_index = self.hash_prime(remapped_index as usize, evicted_key);
                    // println!("{} {}", r, remapped_index);
                    // insert key in correct position,
                    block = &mut self.block_store[(remapped_index/BUCKETS_PER_BLOCK) as usize];
                    local_index = remapped_index % BUCKETS_PER_BLOCK;

                    // calculate offset
                    offset = 0;
                    for i in 0..local_index {
                        offset += block.fca[i as usize];
                    }
                    //check FCA isn't full
                    let check = block.fca[local_index as usize];
                    if offset + check >= BUCKETS_PER_BLOCK as u8 || check >= FCA_MAX_VAL {
                        println!("need to evict again");
                    }
                    else {
                        // println!("evicted successfully");
                        // insert into fsa
                        let mut fsa= &mut block.fsa;
                        fsa.insert((offset + check) as usize, f);

                        // update fca
                        let mut fca = &mut block.fca;
                        fca[local_index as usize] += 1;
                        finished = true;
                        break;
                    }
                }
     */







    pub fn member(&self, x: u64) -> bool {
        // obtain indices
        let f = self.fingerprint(x);
        let glbi1 = self.hash1(x);
        let mut block1 = &self.block_store[(glbi1/BUCKETS_PER_BLOCK) as usize];
        let lbi1 = glbi1 % BUCKETS_PER_BLOCK;

        // calculate offset.
        let mut off = 0;
        for i in 0..lbi1 {
            off += block1.fca[i as usize];
        }
        // println!("{}", off);
        //
        let mut num_in_bucket = block1.fca[lbi1 as usize];
        for j in off as usize..=(off+num_in_bucket) as usize {
            if block1.fsa[j] == f {
                return true;
            }
        }

        // we haven't matched yet. check if overflow bit is set.
        if block1.ota[utils::map(lbi1 as u64, 16) as usize]  < 1 {
            return false;
        }

        // fingerprint might be in other bucket

        let glbi2 = self.hash2(glbi1 + lbi1, f);
        let mut block2 = &self.block_store[(glbi2/BUCKETS_PER_BLOCK) as usize];
        let lbi2 = glbi2 % BUCKETS_PER_BLOCK;

        // calculate offset
        // perform lookup and see if exists.
        let mut off2 = 0;
        let mut i = 0;
        for i in 0..lbi2 {
            off2 += block2.fca[i as usize];
        }
        let mut num_in_bucket = block2.fca[lbi2 as usize];

        for j in off2..=off2+num_in_bucket {
            if block2.fsa[j as usize] == f {
                return true;
            }
        }
        return false;
    }

    // pub fn delete(&mut self, x: u64) -> bool {
    //     // obtain indices
    //     let f = self.fingerprint(x);
    //     let glbi1 = self.hash1(x);
    //     let mut block1 = &mut self.block_store[(glbi1/BUCKETS_PER_BLOCK) as usize];
    //     let lbi1 = glbi1 % BUCKETS_PER_BLOCK;
    //
    //     // calculate offset.
    //     let mut off: usize = 0;
    //     let mut i: usize = 0;
    //     loop {
    //         if i > (2 * lbi1) as usize {
    //             break;
    //         }
    //         off += ((block1.fca.member(i as u64) as u32 & 1u32) + (2* (block1.fca.member((i + 1) as u64) as u32 & 1u32))) as usize;
    //         i+=2;
    //     }
    //     let mut num_in_bucket = block1.fca.member(2 * lbi1 as u64) as u32 & 1u32 + 2 * (block1.fca.member(((2 * lbi1) + 1) as u64) as u32 & 1u32);
    //     off = off.saturating_sub(2);
    //     off = off.saturating_sub(num_in_bucket as usize);
    //     let mut fsa = &mut block1.fsa;
    //     // println!("{}", f);
    //     for j in off..=off+num_in_bucket as usize +2 {
    //         if fsa[j] == f {
    //             // println!("deleted!!! : {}", f);
    //             fsa.remove(j);
    //             // decrement FCA counter.
    //             let mut fca = &mut block1.fca;
    //             if fca.member((2 * lbi1) as u64) {
    //                 if fca.member((2u64 * lbi1 as u64) + 1) {
    //                     fca.delete(2 * lbi1 as u64);
    //                 }
    //                 else {
    //                     fca.delete((2 * lbi1) as u64);
    //                 }
    //             }
    //             else {
    //                 if fca.member((2u64 * lbi1 as u64) + 1) {
    //                     fca.delete((2u64 * lbi1 as u64) + 1);
    //                     fca.insert((2 * lbi1) as u64);
    //                 }
    //             }
    //             return true;
    //         }
    //     }
    //
    //     // fingerprint might be in other bucket
    //
    //     let glbi2 = self.hash2(glbi1 + lbi1, f);
    //     let mut block2 = &mut self.block_store[(glbi2/BUCKETS_PER_BLOCK) as usize];
    //     let lbi2 = glbi2 % BUCKETS_PER_BLOCK;
    //
    //     // calculate offset
    //     let mut off2: usize = 0;
    //     let mut i = 0;
    //     loop {
    //         if i > 2 * lbi2 {
    //             break;
    //         }
    //         off2 += ((block2.fca.member(i as u64) as u32 & 1u32) + (2* (block2.fca.member((i + 1) as u64) as u32 & 1u32))) as usize;
    //         i+=2
    //     }
    //     let mut num_in_bucket = (block2.fca.member(2 * (lbi2) as u64) as u32 & 1u32 + 2 * (block2.fca.member(((2 * (lbi2)) + 1) as u64) as u32 & 1u32)) as usize;
    //     off2 = off2.saturating_sub(2);
    //     off2 = off2.saturating_sub(num_in_bucket);
    //
    //     let mut fsa = &mut block2.fsa;
    //     for j in off2..=off2+num_in_bucket+2 {
    //         // println!("deleted!!! : {}", f);
    //         fsa.remove(j);
    //         // decrement FCA counter.
    //         let mut fca = &mut block2.fca;
    //         if fca.member((2 * lbi1) as u64) {
    //             if fca.member((2u64 * lbi1 as u64) + 1) {
    //                 // cannot happen
    //                 fca.delete(2 * lbi1 as u64);
    //             }
    //             else {
    //                 fca.delete((2 * lbi1) as u64);
    //             }
    //         }
    //         else {
    //             if fca.member((2u64 * lbi1 as u64) + 1) {
    //                 fca.delete((2u64 * lbi1 as u64) + 1);
    //                 fca.insert((2 * lbi1) as u64);
    //             }
    //         }
    //         return true;
    //     }
    //     //element not here, return false as deletion unsuccessful.
    //     return false;
    // }

    pub(crate) fn fingerprint(&self, key: u64) -> u8 {
        return hash(key,8, self.hashes[0].0, self.hashes[0].1, self.hashes[0].2) as u8;
    }

    fn base_hash(&self, key: u64) -> u32 {
        return hash(key,self.l, self.hashes[1].0, self.hashes[1].1, self.hashes[1].2);
    }

    pub(crate) fn hash1(&self, key: u64) -> u32 {
        return utils::map(self.base_hash(key) as u64, self.size) as u32;
    }

    pub(crate) fn hash2(&self, h1: u32, fingerprint: u8) -> u32 {
        let y = -1_i32;
        // return self.hash_prime(h1 as usize,fingerprint);
        return utils::map_neg((h1 as i32 + (y.pow(h1 & 1) * self.offset(fingerprint) as i32)), self.size) as u32;
    }

    pub(crate) fn hash_prime(&self, beta: usize, fingerprint: u8) -> u32 {
        let y = -1_i32;
        return utils::map_neg((beta as i32 + (y.pow((beta & 1) as u32) * self.offset(fingerprint) as i32)), self.size) as u32;
    }

    pub(crate) fn offset(&self, fingerprint: u8) -> u32 {
        return (BUCKETS_PER_BLOCK + (fingerprint as u32 % OFF_RANGE)) | 1u32;
    }

    fn hash_ota(&self, key: u64) -> usize {
        return (hash(key, 4, self.hashes[3].0, self.hashes[3].1, self.hashes[3].2) % 16) as usize;
    }

}