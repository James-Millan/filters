#[path = "bloomfilter.rs"]
mod bloomfilter;
use std::f64;
use std::ptr::null;
use bloomfilter::BloomFilter;
use rand::Rng;
use slab::Slab;

#[path = "utils.rs"]
mod utils;

pub struct BlockedBloomFilter {
    size: u64,
    blocks: Slab<Vec<u8>>,
    block_size: usize,
    num_blocks: u64,
    hash_functions: Vec<(u64,u64,u64)>,
    binary_info: (u32,u32)
}

impl BlockedBloomFilter {
    // block_size = size of cache line in bytes.
    pub fn new(expected_inserts : u64, block_size: usize, false_positive_rate: f64) -> Self {
        let size: u64 = ((-1.44 * (expected_inserts as f64)).ceil()
            * false_positive_rate.log2() + 0.5) as u64 ;
        let num_hashes = (-false_positive_rate.log2() + 0.5) as usize;
        let num_blocks = (size + ((block_size*8) - 1) as u64) / (block_size*8) as u64;

        let mut rng = rand::thread_rng();
        let a1 = rng.gen_range(1..=u64::MAX);
        let a2 = rng.gen_range(1..=u64::MAX);
        let b = rng.gen_range(1..=u64::MAX);
        let pair = (64 - (num_blocks - 1).leading_zeros(), 64 - (block_size - 1).leading_zeros());
        BlockedBloomFilter {
            size,
            blocks: Self::generate_blocks(num_blocks, block_size),
            block_size,
            num_blocks,
            // first hash function is always to find the block.
            hash_functions: Self::generate_hash_functions(num_hashes),
            binary_info: pair
        }
    }


    fn generate_hash_functions(num_hashes: usize) -> Vec<(u64, u64,u64)> {
        let mut rng = rand::thread_rng();
        let mut hash_functions = Vec::new();

        for _ in 0..num_hashes {
            let a1: u64 = rng.gen_range(1..=u64::MAX );
            let a2: u64 = rng.gen_range(1..=u64::MAX);
            let b: u64 = rng.gen_range(1..=u64::MAX);
            hash_functions.push((a1,a2,b));
        }
        return hash_functions;
    }
    fn generate_blocks(num_blocks: u64, block_size: usize) -> Slab<Vec<u8>> {
        let mut slab: Slab<Vec<u8>> = Slab::new();
        let n = num_blocks;
        for i in 0..=n {
            let block = vec![0; block_size];
            slab.insert(block);
        }
        return slab;
    }

    fn get_block_id(&self, element: u64) -> usize {
        // need binary log of the number of blocks here.
        return (utils::hash(element, self.binary_info.0 as u32, self.hash_functions[0].0, self.hash_functions[0].1,
                           self.hash_functions[0].2) as usize ) % self.num_blocks as usize;
    }

    // Add an element to the correct block.
    pub fn insert(&mut self, element: u64) {
        let block_id = self.get_block_id(element);
        let block = self.blocks.get_mut(block_id).unwrap();

        for hash_function in &self.hash_functions {
            let index : u64 = (utils::hash(element, self.binary_info.1, hash_function.0, hash_function.1,
                                           hash_function.2) % self.block_size as u32) as u64;
            block[(index / 8) as usize] |= 1 << (index % 8);
        }
    }

    // Check if an element is present in the correct block.
    pub fn member(&mut self, element: u64) -> bool {
        let block_id = self.get_block_id(element);
        let block = self.blocks.get_mut(block_id).unwrap();

        for hash_function in &self.hash_functions {
            let index : u64 = (utils::hash(element, self.binary_info.1, hash_function.0, hash_function.1,
                                           hash_function.2) % self.block_size as u32) as u64;
            let mask = 1 << (index % 8);
            if !((block[(index / 8) as usize] & mask)!= 0) {
                return false;
            }
        }
        return true;
    }
}

