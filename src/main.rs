mod bloomfilter;
mod cuckoofilter;
mod countingbloomfilter;

extern crate rand;
use rand::Rng;

fn main() {

    let size = 100000;
    let num_hashes = 5;
    let mut bloom_filter = crate::bloomfilter::BloomFilter::new(size, num_hashes);

    bloom_filter.insert(25);
    bloom_filter.insert(2);
    bloom_filter.insert(10);

    // println!("Contains '25': {}", bloom_filter.member(25));
    // println!("Contains '11': {}", bloom_filter.member(11));
    // println!("Contains '2': {}", bloom_filter.member(2));
    // println!("Contains '10': {}", bloom_filter.member(10));

    //println!("Bit Array: {:?}", bloom_filter.bit_array);


    // let size = 100;
    // let num_hashes = 10;
    // let mut counting_bloom_filter = crate::countingbloomfilter::CountingBloomFilter::new(size,num_hashes);
    // counting_bloom_filter.insert(24);
    // counting_bloom_filter.insert(12);
    // counting_bloom_filter.insert(98);
    //
    // println!("Contains '24': {}", counting_bloom_filter.member(24));
    // println!("Contains '12': {}", counting_bloom_filter.member(12));
    // println!("Contains '11': {}", counting_bloom_filter.member(11));
    //
    // counting_bloom_filter.delete(12);
    // println!("Contains '12': {}", counting_bloom_filter.member(12));
    // println!("Contains '2': {}", counting_bloom_filter.member(2));
    //
    // println!("Bit Array: {:?}", counting_bloom_filter.count_array);



    let mut cuckoo_filter = crate::cuckoofilter::CuckooFilter::new(1000,100000);
    for i in 0..=100 {
        cuckoo_filter.insert(i);
    }

    for j in 0..=100 {
        println!("Contains '{}': {}", j,cuckoo_filter.member(j));
    }
    println!("Contains '12': {}", cuckoo_filter.member(12));
    println!("Buckets: {:?}", cuckoo_filter.buckets);


}
