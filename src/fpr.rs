use crate::blockedbloomfilter::BlockedBloomFilter;
use crate::bloomfilter::BloomFilter;
use crate::countingbloomfilter::CountingBloomFilter;
use crate::cuckoofilter::CuckooFilter;
use crate::registeralignedbloomfilter::RegisterAlignedBloomFilter;
use crate::xorfilter;

pub(crate) fn bloom_filter_fpr(size: u64, fpr:f64, keys: &Vec<u64>) {
    let mut bloom_filter = BloomFilter::new(size, fpr);
    for key in keys {
        bloom_filter.insert(*key);
    }
    println!("finished inserting keys");
    let mut count: f64 = 0f64;
    let mut fp: f64 = 0f64;
    for i in size+1..=10*size {
        count += 1.0f64;
        if (bloom_filter.member(i)) {
            fp += 1.0f64;
        }
    }
    let fpr = fp/count;
    println!("Bloom filter fpr: '{}'", fpr);
}

pub(crate) fn counting_bloom_filter_fpr(size: u64, fpr:f64, keys: &Vec<u64>) {
    let mut counting_bloom_filter = CountingBloomFilter::new(size, fpr);
    for key in keys {
        counting_bloom_filter.insert(*key);
    }
    let mut count: f64 = 0f64;
    let mut fp: f64 = 0f64;
    for i in size+1..=10*size {
        count += 1.0f64;
        if (counting_bloom_filter.member(i)) {
            fp += 1.0f64;
        }
    }
    let fpr = fp/count;
    println!("Counting Bloom filter fpr: '{}'", fpr);
}

//TODO update params
pub(crate) fn cuckoo_filter_fpr(size: u64, fpr:f64, keys: &Vec<u64>) {
    let mut cuckoo_filter = CuckooFilter::new(size as usize, 10, 8);
    for key in keys {
        cuckoo_filter.insert(*key);
    }
    let mut count: f64 = 0f64;
    let mut fp: f64 = 0f64;
    for i in size+1..=10*size {
        count += 1.0f64;
        if (cuckoo_filter.member(i)) {
            fp += 1.0f64;
        }
    }
    let fpr = fp/count;
    println!("Cuckoo filter fpr: '{}'", fpr);
}

pub(crate) fn xor_filter_fpr(keys: &Vec<u64>) {
    let mut xor_filter = xorfilter::XorFilter::new(keys.clone());
    let mut count: f64 = 0f64;
    let mut fp: f64 = 0f64;
    for i in keys.len()+1..=10*keys.len() {
        count += 1.0f64;
        if (xor_filter.member(i as u64)) {
            fp += 1.0f64;
        }
    }
    let fpr = fp/count;
    println!("Xor filter fpr: '{}'", fpr);
}


// TODO update params
// pub(crate) fn blocked_bloom_filter_fpr(size: u64, fpr:f64, keys: Vec<u64>) {
//     let mut blocked_bloom_filter = BlockedBloomFilter::new(size, fpr as usize);
//     for key in &keys {
//         blocked_bloom_filter.insert(*key);
//     }
//     println!("finished inserting keys");
//     let mut count: f64 = 0f64;
//     let mut fp: f64 = 0f64;
//     for i in 0..=10*size {
//         println!("'{}'", i);
//         if !keys.contains(&i) {
//             count += 1.0f64;
//             if (blocked_bloom_filter.member(i)) {
//                 fp += 1.0f64;
//             }
//         }
//     }
//     let fpr = fp/count;
//     println!("Blocked Bloom filter fpr: '{}'", fpr);
// }
//
// pub(crate) fn register_aligned_bloom_filter_fpr(size: u64, fpr:f64, keys: Vec<u64>) {
//     let mut register_aligned_bloom_filter = RegisterAlignedBloomFilter::new(size, fpr);
//     for key in &keys {
//         register_aligned_bloom_filter.insert(*key);
//     }
//     println!("finished inserting keys");
//     let mut count: f64 = 0f64;
//     let mut fp: f64 = 0f64;
//     for i in 0..=10 * size {
//         println!("'{}'", i);
//         if !keys.contains(&i) {
//             count += 1.0f64;
//             if (register_aligned_bloom_filter.member(i)) {
//                 fp += 1.0f64;
//             }
//         }
//     }
//     let fpr = fp / count;
//     println!("Counting Bloom filter fpr: '{}'", fpr);
// }