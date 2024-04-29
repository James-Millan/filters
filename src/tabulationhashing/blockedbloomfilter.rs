use std::f64;

#[path = "tabulationhashing.rs"]
mod tabulationhashing;

use rand::Rng;
use slab::Slab;

#[path = "utils.rs"]
mod utils;

pub struct BlockedBloomFilter {
    size: u64,
    blocks: Slab<Vec<u8>>,
    block_size: usize,
    num_blocks: u64,
    hasher: TabulationHashing,
    hashers: Vec<TabulationHashing>,
    binary_info: (u32,u32)
}

impl BlockedBloomFilter {
    // block_size = size of cache line in bytes.
    pub fn new(expected_inserts : u64, block_size: usize, false_positive_rate: f64) -> Self {
        let size: u64 = ((-1.44 * (expected_inserts as f64)).ceil()
            * (false_positive_rate/5.0).log2() + 0.5) as u64 ;
        let num_hashes = (-false_positive_rate.log2() + 0.5) as usize;
        let num_blocks = (size + ((block_size*8) - 1) as u64) / (block_size*8) as u64;
        let mut hashers = vec![];
        for i in num_hashes {
            hashers.push(tabulationhashing::TabulationHashing::new());
        }
        let pair = (utils::log_base(num_blocks as f64, 2f64) as u32, utils::log_base(block_size as f64, 2f64) as u32);
        BlockedBloomFilter {
            size,
            blocks: Self::generate_blocks(num_blocks, block_size),
            block_size,
            num_blocks,
            // first hash function is always to find the block.
            hasher: tabulationhashing::TabulationHashing::new(),
            binary_info: pair
        }
    }
    fn generate_blocks(num_blocks: u64, block_size: usize) -> Slab<Vec<u8>> {
        let mut slab: Slab<Vec<u8>> = Slab::new();
        let n = num_blocks;
        for _i in 0..=n {
            let block = vec![0; block_size];
            slab.insert(block);
        }
        return slab;
    }

    fn get_block_id(&self, element: u64) -> usize {
        // need binary log of the number of blocks here.
        if(self.num_blocks <= 1) {
            return 0;
        }
        return self.hasher.tabulation_hashing() % self.num_blocks as usize;
    }

    // Add an element to the correct block.
    pub fn insert(&mut self, element: u64) {
        let block_id = self.get_block_id(element);
        let block = self.blocks.get_mut(block_id).unwrap();

        for hash in &self.hashers {
            let index : u64 = (hash.tabulation_hashing() % self.block_size as u64);
            block[(index / 8) as usize] |= 1 << (index % 8);
        }
    }

    // Check if an element is present in the correct block.
    pub fn member(&mut self, element: u64) -> bool {
        let block_id = self.get_block_id(element);
        let block = self.blocks.get_mut(block_id).unwrap();

        for hasher in &self.hashers {
            let index : u64 = (hasher.tabulation_hashing() % self.block_size as u64);
            let mask = 1 << (index % 8);
            if !((block[(index / 8) as usize] & mask)!= 0) {
                return false;
            }
        }
        return true;
    }
}

