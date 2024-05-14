use std::f64;
use rand::Rng;
use slab::Slab;
#[path = "../bitvector.rs"]
mod bitvector;

#[path = "../utils.rs"]
mod utils;

#[path = "tabulationhashing.rs"]
mod tabulationhashing;

use tabulationhashing::TabulationHashing;


pub struct BlockedBloomFilter {
    size: u64,
    blocks: Slab<Vec<u8>>,
    block_size: usize,
    num_blocks: u64,
    num_hashes: usize,
    hash_functions: Vec<TabulationHashing>,
}

impl BlockedBloomFilter {
    // block_size = size of cache line in bytes.
    pub fn new(expected_inserts : u64, block_size: usize, false_positive_rate: f64) -> Self {
        let mut size: u64 = ((-1.44 * (expected_inserts as f64)).ceil()
            * (false_positive_rate/5.0).log2() + 0.5) as u64 ;
        size = size * 100;
        let num_hashes = (-false_positive_rate.log2() + 1.5) as usize;
        let num_blocks = (size + ((block_size*8) - 1) as u64) / (block_size*8) as u64;
        BlockedBloomFilter {
            size,
            blocks: Self::generate_blocks(num_blocks, block_size),
            block_size,
            num_blocks,
            num_hashes,
            // first hash function is always to find the block.
            hash_functions: Self::generate_hash_functions(num_hashes)
        }
    }
    fn generate_hash_functions(num_hashes: usize) -> Vec<TabulationHashing> {
        let mut hash_functions = Vec::new();
        for _ in 0..num_hashes {
            hash_functions.push(TabulationHashing::new());
        }
        return hash_functions;
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
        return ((self.hash_functions[0].tabulation_hashing(element)) % self.num_blocks) as usize;
    }

    // Add an element to the correct block.
    pub fn insert(&mut self, element: u64) {
        let block_id = self.get_block_id(element);
        let block = self.blocks.get_mut(block_id).unwrap();

        for i in 1..self.num_hashes {
            let hash_function = &self.hash_functions[i];
            let index : u64 = hash_function.tabulation_hashing(element) % self.block_size as u64;
            block[(index / 8) as usize] |= 1 << (index % 8);
        }
    }

    // Check if an element is present in the correct block.
    pub fn member(&self, element: u64) -> bool {
        let block_id = self.get_block_id(element);
        let block = self.blocks.get(block_id).unwrap();

        for i in 1..self.num_hashes {
            let hash_function = &self.hash_functions[i];
            let index : u64 = hash_function.tabulation_hashing(element) % self.block_size as u64;
            let mask = 1 << (index % 8);
            if !((block[(index / 8) as usize] & mask)!= 0) {
                return false;
            }
        }
        return true;
    }
}

