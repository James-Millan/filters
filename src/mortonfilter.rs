use rand::Rng;

#[path = "mortonblock.rs"]
mod mortonblock;
use mortonblock::MortonBlock;
#[path = "utils.rs"]
mod utils;
use utils::{hash};

const BUCKETS_PER_BLOCK: u32 = 46;
const OFF_RANGE: u32 = 64;

pub struct MortonFilter {
    block_store: Vec<MortonBlock>,
    hashes: Vec<(u64,u64,u64)>,
    cache_size: usize,
    size: u64,
    l: u32,
}

impl MortonFilter {
    pub fn new(size : u64, fpr: f64) -> MortonFilter {
        let length = (2f64 * size as f64) as u64;
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
        for i in 0..length {
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
        let mut i = 0;
        loop {
            if (i > 2* lbi1) {
                break;
            }
            off += ((block1.fca.member(i as u64) as u32 & 1u32) + (2* (block1.fca.member((i + 1) as u64) as u32 & 1u32)));
            i+=2
        }
        //overflow condition
        // calculate FCA value. must be less than 3
        let overflow_check = ((block1.fca.member(2 * lbi1 as u64) as u32 & 1u32) + (2* (block1.fca.member((2 * lbi1 + 1) as u64) as u32 & 1u32)));
        if off >= BUCKETS_PER_BLOCK || overflow_check >= 3 {
            // set OTA
            let index = utils::map(lbi1 as u64, 32);
            let mut ota = &mut block1.ota;
            ota.insert(index);
            let glbi2 = self.hash2(x, f);
            let mut block2: &mut MortonBlock  = &mut self.block_store[(glbi2/BUCKETS_PER_BLOCK) as usize];
            let lbi2 = glbi2 % BUCKETS_PER_BLOCK;

            // calculate offset
            // perform lookup and see if exists.
            let mut off2 = 0;
            let mut i = 0;
            loop {
                if i > 2 * lbi2 {
                    break;
                }
                off2 += ((block2.fca.member(i as u64) as u32 & 1u32) + (2* (block2.fca.member((i + 1) as u64) as u32 & 1u32)));
                i+=2
            }
            //check FCA isn't full
            let overflow_check = ((block2.fca.member(2 * lbi2 as u64) as u32 & 1u32) + (2* (block2.fca.member((2 * lbi2 + 1) as u64) as u32 & 1u32)));
            if off2 >= BUCKETS_PER_BLOCK || overflow_check >= 3 {
                // perform eviction
                // println!("eviction needed, {}", x);
                let mut num_kicks = 0;
                let mut finished = false;
                let mut offset = off;
                let mut block: &mut MortonBlock = &mut self.block_store[(glbi1/BUCKETS_PER_BLOCK) as usize];
                let mut f = f;
                let mut local_index = lbi1;


                while !finished && num_kicks < 1000 {
                    num_kicks += 1;
                    // select a key to evict and update fsa with new key.
                    let mut fsa: &mut Vec<u8> = &mut block.fsa;
                    let evicted_key = fsa[offset as usize];
                    //println!("eviction needed, {}, {}", x, evicted_key);

                    fsa[offset as usize] = f;
                    f = evicted_key;

                    // update ota to mark eviction
                    let index = utils::map(local_index as u64, 32);
                    let mut ota = &mut block.ota;
                    ota.insert(index);

                    //remap evicted key using hash_prime
                    let remapped_index = self.hash_prime(offset as usize, evicted_key);

                    // insert key in correct position,
                    block = &mut self.block_store[(remapped_index/BUCKETS_PER_BLOCK) as usize];
                    local_index = remapped_index % BUCKETS_PER_BLOCK;

                    // calculate offset
                    offset = 0;
                    let mut i = 0;
                    loop {
                        if i > 2 * local_index {
                            break;
                        }
                        offset += ((block.fca.member(i as u64) as u32 & 1u32) + (2* (block.fca.member((i + 1) as u64) as u32 & 1u32)));
                        i+=2
                    }
                    //check FCA isn't full
                    let check = ((block.fca.member(2 * local_index as u64) as u32 & 1u32) + (2* (block.fca.member((2 * local_index + 1) as u64) as u32 & 1u32)));
                    if offset >= BUCKETS_PER_BLOCK || check >= 3 {
                        //println!("need to evict again");
                    }
                    else {
                        // println!("evicted successfully");
                        // insert into fsa
                        let mut fsa= &mut block.fsa;
                        fsa.insert(offset as usize, evicted_key);

                        // update fca
                        let mut fca = &mut block.fca;
                        if fca.member((2 * local_index) as u64) {
                            if fca.member((2u64 * local_index as u64) + 1) {
                                // cannot happen
                                // println!("overflow");
                            }
                            else {
                                fca.insert((2u64 * local_index as u64) + 1);
                                fca.delete((2 * local_index) as u64);
                            }
                        }
                        else {
                            fca.insert((2 * local_index) as u64);
                        }
                        finished = true;
                    }
                }
            }
            else {
                // insert into fsa
                let mut fsa: &mut Vec<u8> = &mut block2.fsa;
                fsa.insert(off2 as usize, f);
                // update fca
                let mut fca = &mut block2.fca;
                if fca.member((2 * lbi2) as u64) {
                    if fca.member((2u64 * lbi2 as u64) + 1) {
                        // cannot happen
                        // println!("overflow");
                    }
                    else {
                        fca.insert((2u64 * lbi2 as u64) + 1);
                        fca.delete((2 * lbi2) as u64);
                    }
                }
                else {
                    fca.insert((2 * lbi2) as u64);
                }
                return true;
            }
        }
        else {
            // insert into fsa
            let mut fsa= &mut block1.fsa;
            fsa.insert(off as usize, f);

            // update fca
            let mut fca = &mut block1.fca;
            if fca.member((2 * lbi1) as u64) {
                if fca.member((2u64 * lbi1 as u64) + 1) {
                    // cannot happen
                    // println!("overflow");
                }
                else {
                    fca.insert((2u64 * lbi1 as u64) + 1);
                    fca.delete((2 * lbi1) as u64);

                }
            }
            else {
                fca.insert((2 * lbi1) as u64);
            }

            return true;
        }
        return false;
    }

    pub fn member(&self, x: u64) -> bool {
        // obtain indices
        let f = self.fingerprint(x);
        let glbi1 = self.hash1(x);
        let mut block1 = &self.block_store[(glbi1/BUCKETS_PER_BLOCK) as usize];
        let lbi1 = glbi1 % BUCKETS_PER_BLOCK;

        // calculate offset.
        let mut off: usize = 0;
        let mut i: usize = 0;
        loop {
            if i > (2 * lbi1) as usize {
                break;
            }
            off += ((block1.fca.member(i as u64) as u32 & 1u32) + (2* (block1.fca.member((i + 1) as u64) as u32 & 1u32))) as usize;
            i+=2;
        }
        // println!("{}", off);
        //
        let mut num_in_bucket = block1.fca.member(2 * lbi1 as u64) as u32 & 1u32 + 2 * (block1.fca.member(((2 * lbi1) + 1) as u64) as u32 & 1u32);
        // println!("{:?}, {}, {}, {}, {}", block1.fsa, f, off, num_in_bucket, block1.fsa[off]);
        // if num_in_bucket == 0 {
        //     println!("mistake in num in bucket v1");
        //     num_in_bucket = 1;
        // }
        off = off.saturating_sub(2);
        off = off.saturating_sub(num_in_bucket as usize);

        for j in off..=(off+num_in_bucket as usize + 2) {
            if block1.fsa[j] == f {
                return true;
            }
        }

        // // we haven't matched yet. check overflow.
        // if !block1.ota.member(self.hash_ota(x) as u64) {
        //     return false;
        // }

        // fingerprint might be in other bucket

        let glbi2 = self.hash2(x, f);
        let mut block2 = &self.block_store[(glbi2/BUCKETS_PER_BLOCK) as usize];
        let lbi2 = glbi2 % BUCKETS_PER_BLOCK;

        // calculate offset
        // perform lookup and see if exists.
        let mut off2: usize = 0;
        let mut i = 0;
        loop {
            if i > 2 * lbi2 {
                break;
            }
            off2 += ((block2.fca.member(i as u64) as u32 & 1u32) + (2* (block2.fca.member((i + 1) as u64) as u32 & 1u32))) as usize;
            i+=2
        }
        let mut num_in_bucket = (block2.fca.member(2 * (lbi2) as u64) as u32 & 1u32 + 2 * (block2.fca.member(((2 * (lbi2)) + 1) as u64) as u32 & 1u32)) as usize;
        off2 = off2.saturating_sub(2);
        off2 = off2.saturating_sub(num_in_bucket);

        // if num_in_bucket == 0 {
        //     println!("mistake in num in bucket");
        //     num_in_bucket = 1;
        // }
        // println!("{:?}, {}, {},{}", block2.fsa, f, off2, num_in_bucket);
        // num in bucket is incorrect.

        for j in off2..=off2+num_in_bucket+2 {
            if block2.fsa[j] == f {
                return true;
            }
        }

        // ------------------------------------------------------------------------------

        let f = self.fingerprint(x);
        let glbi3 = self.hash_prime(lbi1 as usize, f);
        let mut block3 = &self.block_store[(glbi3/BUCKETS_PER_BLOCK) as usize];
        let lbi3 = glbi3 % BUCKETS_PER_BLOCK;

        // calculate offset.
        let mut off3: usize = 0;
        let mut i: usize = 0;
        loop {
            if i > (2 * lbi3) as usize {
                break;
            }
            off3 += ((block3.fca.member(i as u64) as u32 & 1u32) + (2* (block3.fca.member((i + 1) as u64) as u32 & 1u32))) as usize;
            i+=2;
        }
        // println!("{}", off);
        //
        let mut num_in_bucket = block3.fca.member(2 * lbi3 as u64) as u32 & 1u32 + 2 * (block3.fca.member(((2 * lbi3) + 1) as u64) as u32 & 1u32);
        // println!("{:?}, {}, {}, {}, {}", block1.fsa, f, off, num_in_bucket, block1.fsa[off]);
        // if num_in_bucket == 0 {
        //     println!("mistake in num in bucket v1");
        //     num_in_bucket = 1;
        // }
        off3 = off3.saturating_sub(2);
        off3 = off3.saturating_sub(num_in_bucket as usize);

        for j in off3..=(off3+num_in_bucket as usize + 2) {
            if block3.fsa[j] == f {
                // println!("-------------------------------------------------------------------");
                return true;
            }
        }
        // everything check has failed. item not in filter.
        // if block2.fsa.contains(&f) {
        //    println!("block 2 has it");
        //    println!("{:?}, {}, {}, {}, {}", block2.fsa, f, off2, num_in_bucket, block2.fsa[off2]);
        //
        // }
        // else if block1.fsa.contains(&f) {
        //     println!("block 1 has it");
        //     println!("{:?}, {}, {}, {}, {}", block1.fsa, f, off, num_in_bucket, block1.fsa[off]);
        //
        // }
        // else {
        //     println!("it is not here!");
        // }
        return false;
    }

    pub fn delete(&mut self, x: u64) -> bool {
        // obtain indices
        let f = self.fingerprint(x);
        let glbi1 = self.hash1(x);
        let mut block1 = &mut self.block_store[(glbi1/BUCKETS_PER_BLOCK) as usize];
        let lbi1 = glbi1 % BUCKETS_PER_BLOCK;

        // calculate offset.
        let mut off: usize = 0;
        let mut i: usize = 0;
        loop {
            if i > (2 * lbi1) as usize {
                break;
            }
            off += ((block1.fca.member(i as u64) as u32 & 1u32) + (2* (block1.fca.member((i + 1) as u64) as u32 & 1u32))) as usize;
            i+=2;
        }
        let mut num_in_bucket = block1.fca.member(2 * lbi1 as u64) as u32 & 1u32 + 2 * (block1.fca.member(((2 * lbi1) + 1) as u64) as u32 & 1u32);
        off = off.saturating_sub(2);
        off = off.saturating_sub(num_in_bucket as usize);
        let mut fsa = &mut block1.fsa;
        // println!("{}", f);
        for j in off..=off+num_in_bucket as usize +2 {
            if fsa[j] == f {
                // println!("deleted!!! : {}", f);
                fsa.remove(j);
                // decrement FCA counter.
                let mut fca = &mut block1.fca;
                if fca.member((2 * lbi1) as u64) {
                    if fca.member((2u64 * lbi1 as u64) + 1) {
                        fca.delete(2 * lbi1 as u64);
                    }
                    else {
                        fca.delete((2 * lbi1) as u64);
                    }
                }
                else {
                    if fca.member((2u64 * lbi1 as u64) + 1) {
                        fca.delete((2u64 * lbi1 as u64) + 1);
                        fca.insert((2 * lbi1) as u64);
                    }
                }
                return true;
            }
        }

        // fingerprint might be in other bucket

        let glbi2 = self.hash2(x, f);
        let mut block2 = &mut self.block_store[(glbi2/BUCKETS_PER_BLOCK) as usize];
        let lbi2 = glbi2 % BUCKETS_PER_BLOCK;

        // calculate offset
        let mut off2: usize = 0;
        let mut i = 0;
        loop {
            if i > 2 * lbi2 {
                break;
            }
            off2 += ((block2.fca.member(i as u64) as u32 & 1u32) + (2* (block2.fca.member((i + 1) as u64) as u32 & 1u32))) as usize;
            i+=2
        }
        let mut num_in_bucket = (block2.fca.member(2 * (lbi2) as u64) as u32 & 1u32 + 2 * (block2.fca.member(((2 * (lbi2)) + 1) as u64) as u32 & 1u32)) as usize;
        off2 = off2.saturating_sub(2);
        off2 = off2.saturating_sub(num_in_bucket);

        let mut fsa = &mut block2.fsa;
        for j in off2..=off2+num_in_bucket+2 {
            // println!("deleted!!! : {}", f);
            fsa.remove(j);
            // decrement FCA counter.
            let mut fca = &mut block2.fca;
            if fca.member((2 * lbi1) as u64) {
                if fca.member((2u64 * lbi1 as u64) + 1) {
                    // cannot happen
                    fca.delete(2 * lbi1 as u64);
                }
                else {
                    fca.delete((2 * lbi1) as u64);
                }
            }
            else {
                if fca.member((2u64 * lbi1 as u64) + 1) {
                    fca.delete((2u64 * lbi1 as u64) + 1);
                    fca.insert((2 * lbi1) as u64);
                }
            }
            return true;
        }
        //element not here, return false as deletion unsuccessful.
        return false;
    }

    pub(crate) fn fingerprint(&self, key: u64) -> u8 {
        return hash(key,8, self.hashes[0].0, self.hashes[0].1, self.hashes[0].2) as u8;
    }

    fn base_hash(&self, key: u64) -> u32 {
        return hash(key,self.l, self.hashes[1].0, self.hashes[1].1, self.hashes[1].2);
    }

    fn hash1(&self, key: u64) -> u32 {
        return utils::map(self.base_hash(key) as u64, self.size) as u32;
    }

    fn hash2(&self, key: u64, fingerprint: u8) -> u32 {
        let h1 = self.hash1(key);
        return utils::map((h1 as i32 + (-1i32.pow(h1 & 1) * self.offset(fingerprint) as i32)) as u64, self.size) as u32;
    }

    fn hash_prime(&self, beta: usize, fingerprint: u8) -> u32 {
        return utils::map((beta as i32 + (-1i32.pow((beta & 1) as u32) * self.offset(fingerprint) as i32)) as u64, self.size) as u32;
    }

    fn offset(&self, fingerprint: u8) -> u32 {
        return (BUCKETS_PER_BLOCK + (fingerprint as u32 % OFF_RANGE)) | 1u32;
    }

    fn hash_ota(&self, key: u64) -> usize {
        return hash(key, 5, self.hashes[3].0, self.hashes[3].1, self.hashes[3].2) as usize;
    }

}