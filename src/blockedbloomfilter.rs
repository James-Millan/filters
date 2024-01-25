#[path = "bloomfilter.rs"]
mod bloomfilter;
use std::f64;
use bloomfilter::BloomFilter;
use rand::Rng;
use slab::Slab;
use crate::utils;

pub struct BlockedBloomFilter {
    expected_insertions: u64,
    filters: Slab<BloomFilter>,
    block_size: usize,
    num_blocks: u64,
    hash_coefficients: (u64,u64,u64),
    fpr_per_block: f64,
}

impl BlockedBloomFilter {
    // block_size = size of cache line in bits.
    pub fn new(expected_insertions : u64, block_size: usize, fpr_per_block: f64) -> Self {
        let mut rng = rand::thread_rng();
        let a1 = rng.gen_range(1..=u64::MAX);
        let a2 = rng.gen_range(1..=u64::MAX);
        let b = rng.gen_range(1..=u64::MAX);
        BlockedBloomFilter {
            expected_insertions,
            filters: Self::create_filters(expected_insertions.div_ceil(block_size as u64), block_size, fpr_per_block),
            block_size,
            num_blocks: expected_insertions.div_ceil(block_size as u64),
            hash_coefficients: (a1,a2,b),
            fpr_per_block,
        }
    }

    fn get_block_id(&self, element: u64) -> usize { 
        return utils::hash(element, self.num_blocks as u32, self.hash_coefficients.0, self.hash_coefficients.1, self.hash_coefficients.2) as usize
    }

    fn create_filters(num_blocks: u64, block_size: usize, fpr_per_block: f64) -> Slab<BloomFilter> {
        let mut slab: Slab<BloomFilter> = Slab::new();
        let n = num_blocks;
        for i in 0..=n {
            let filter = BloomFilter::new(block_size as u64,fpr_per_block);
            slab.insert(filter);
        }
        return slab;
    }

    // Add an element to the correct block.
    pub fn insert(&mut self, element: u64) {
        let block_id = self.get_block_id(element);
        let filter = self.get_or_create_filter(block_id);
        filter.insert(element);
    }

    // Check if an element is present in the BlockedBloomFilter
    pub fn member(&mut self, element: u64) -> bool {
        let block_id = self.get_block_id(element);
        return  self.filters.get_mut(block_id).unwrap().member(element);
    }

    // Internal method to get or create a filter for a given block ID
    fn get_or_create_filter(&mut self, block_id: usize) -> &mut BloomFilter {
        if !self.filters.contains(block_id) {
            let filter = BloomFilter::new(self.block_size as u64,self.fpr_per_block);
            self.filters.insert(filter);
        }
        return self.filters.get_mut(block_id).unwrap();
    }
}

