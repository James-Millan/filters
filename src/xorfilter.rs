// use std::error::Error;
//
// struct Xor8 {
//     seed: u64,
//     block_length: u32,
//     fingerprints: Vec<u8>,
// }
//
// struct Hashes {
//     h: u64,
//     h0: u32,
//     h1: u32,
//     h2: u32,
// }
//
// struct KeyIndex {
//     index: u32,
//     hash: u64,
// }
//
// struct XorSet {
//     xormask: u64,
//     count: u32,
// }
//
// const MAX_ITERATIONS: usize = 1024;
//
// fn murmur64(mut h: u64) -> u64 {
//     h ^= h >> 33;
//     h = h.wrapping_mul(0xff51afd7ed558ccd);
//     h ^= h >> 33;
//     h = h.wrapping_mul(0xc4ceb9fe1a85ec53);
//     h ^= h >> 33;
//     h
// }
//
// fn splitmix64(seed: &mut u64) -> u64 {
//     *seed = seed.wrapping_add(0x9E3779B97F4A7C15);
//     let mut z = *seed;
//     z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
//     z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
//     z ^ (z >> 31)
// }
//
// fn mixsplit(key: u64, seed: u64) -> u64 {
//     murmur64(key.wrapping_add(seed))
// }
//
// fn rotl64(n: u64, c: i32) -> u64 {
//     (n << (c & 63)) | (n >> ((-c) & 63))
// }
//
// fn reduce(hash: u64, n: u32) -> u32 {
//     ((hash.wrapping_mul(n as u64)) >> 32) as u32
// }
//
// fn fingerprint(hash: u64) -> u64 {
//     hash ^ (hash >> 32)
// }
//
// impl Xor8 {
//     fn contains(&self, key: u64) -> bool {
//         let hash = mixsplit(key, self.seed);
//         let f = fingerprint(hash) as u8;
//         let r0 = hash;
//         let r1 = rotl64(hash, 21) as u32;
//         let r2 = rotl64(hash, 42) as u32;
//         let h0 = reduce(r0, self.block_length);
//         let h1 = reduce(r1 as u64, self.block_length) + self.block_length;
//         let h2 = reduce(r2 as u64, self.block_length) + 2 * self.block_length;
//         f == (self.fingerprints[h0 as usize] ^ self.fingerprints[h1 as usize] ^ self.fingerprints[h2 as usize])
//     }
//
//     fn geth0h1h2(&self, k: u64) -> Hashes {
//         let hash = mixsplit(k, self.seed);
//         let mut answer = Hashes {
//             h: hash,
//             h0: 0,
//             h1: 0,
//             h2: 0,
//         };
//         let r0 = hash;
//         let r1 = rotl64(hash, 21);
//         let r2 = rotl64(hash, 42);
//         answer.h0 = reduce(r0, self.block_length);
//         answer.h1 = reduce(r1, self.block_length);
//         answer.h2 = reduce(r2, self.block_length);
//         answer
//     }
//
//     fn geth0(&self, hash: u64) -> u32 {
//         let r0 = hash;
//         reduce(r0, self.block_length)
//     }
//
//     fn geth1(&self, hash: u64) -> u32 {
//         let r1 = rotl64(hash, 21);
//         reduce(r1, self.block_length)
//     }
//
//     fn geth2(&self, hash: u64) -> u32 {
//         let r2 = rotl64(hash, 42);
//         reduce(r2, self.block_length)
//     }
// }
//
// fn scan_count(Qi: &mut Vec<KeyIndex>, setsi: &[XorSet]) -> usize {
//     let mut Qi_size = 0;
//
//     for i in 0..setsi.len() {
//         if setsi[i].count == 1 {
//             Qi[Qi_size].index = i as u32;
//             Qi[Qi_size].hash = setsi[i].xormask;
//             Qi_size += 1;
//         }
//     }
//
//     Qi_size
// }
//
// fn reset_sets(setsi: &mut [XorSet]) {
//     for set in setsi.iter_mut() {
//         set.xormask = 0;
//         set.count = 0;
//     }
// }
//
// fn populate(keys: &[u64]) -> Result<Xor8, Box<dyn Error>> {
//     let size = keys.len();
//     if size == 0 {
//         return Err("provide a non-empty set".into());
//     }
//
//     let capacity = (32.0 + (1.23 * size as f64).ceil()) / 3.0 * 3.0;
//     let mut capacity = capacity as u32;
//
//     capacity = capacity / 3 * 3; // round it down to a multiple of 3
//
//     let mut filter = Xor8 {
//         seed: 0,
//         block_length: capacity / 3,
//         fingerprints: vec![],
//     };
//
//     let mut rng_counter = 1;
//     filter.seed = splitmix64(&mut rng_counter);
//     filter.fingerprints = vec![0; capacity as usize];
//
//     let mut stack = vec![KeyIndex { index: 0, hash: 0 }; size];
//     let mut Q0 = vec![KeyIndex { index: 0, hash: 0 }; filter.block_length as usize];
//     let mut Q1 = vec![KeyIndex { index: 0, hash: 0 }; filter.block_length as usize];
//     let mut Q2 = vec![KeyIndex { index: 0, hash: 0 }; filter.block_length as usize];
//     let mut sets0 = vec![XorSet { xormask: 0, count: 0 }; filter.block_length as usize];
//     let mut sets1 = vec![XorSet { xormask: 0, count: 0 }; filter.block_length as usize];
//     let mut sets2 = vec![XorSet { xormask: 0, count: 0 }; filter.block_length as usize];
//     let mut iterations = 0;
//
//     loop {
//         iterations += 1;
//         if iterations > MAX_ITERATIONS {
//             return Err("too many iterations".into());
//         }
//
//         for i in 0..size {
//             let key = keys[i];
//             let hs = filter.geth0h1h2(key);
//             sets0[hs.h0 as usize].xormask ^= hs.h;
//             sets0[hs.h0 as usize].count += 1;
//             sets1[hs.h1 as usize].xormask ^= hs.h;
//             sets1[hs.h1 as usize].count += 1;
//             sets2[hs.h2 as usize].xormask ^= hs.h;
//             sets2[hs.h2 as usize].count += 1;
//         }
//
//         let (Q0, Q0_size) = (Q0, scan_count(&mut Q0, &sets0));
//         let (Q1, Q1_size) = (Q1, scan_count(&mut Q1, &sets1));
//         let (Q2, Q2_size) = (Q2, scan_count(&mut Q2, &sets2));
//
//         let mut stack_size = 0;
//         while Q0_size + Q1_size + Q2_size > 0 {
//             while Q0_size > 0 {
//                 Q0_size -= 1;
//                 let key_index_var = Q0[Q0_size];
//                 let index = key_index_var.index as usize;
//                 if sets0[index].count == 0 {
//                     continue;
//                 }
//                 let hash = key_index_var.hash;
//                 let h1 = filter.geth1(hash);
//                 let h2 = filter.geth2(hash);
//                 stack[stack_size] = key_index_var;
//                 stack_size += 1;
//                 sets1[h1 as usize].xormask ^= hash;
//                 sets1[h1 as usize].count -= 1;
//                 if sets1[h1 as usize].count == 1 {
//                     Q1[Q1_size].index = h1;
//                     Q1[Q1_size].hash = sets1[h1 as usize].xormask;
//                     Q1_size += 1;
//                 }
//                 sets2[h2 as usize].xormask ^= hash;
//                 sets2[h2 as usize].count -= 1;
//                 if sets2[h2 as usize].count == 1 {
//                     Q2[Q2_size].index = h2;
//                     Q2[Q2_size].hash = sets2[h2 as usize].xormask;
//                     Q2_size += 1;
//                 }
//             }
//             while Q1_size > 0 {
//                 Q1_size -= 1;
//                 let key_index_var = Q1[Q1_size];
//                 let index = key_index_var.index as usize;
//                 if sets1[index].count == 0 {
//                     continue;
//                 }
//                 let hash = key_index_var.hash;
//                 let h0 = filter.geth0(hash);
//                 let h2 = filter.geth2(hash);
//                 let mut key_index_var = key_index_var;
//                 key_index_var.index += filter.block_length;
//                 stack[stack_size] = key_index_var;
//                 stack_size += 1;
//                 sets0[h0 as usize].xormask ^= hash;
//                 sets0[h0 as usize].count -= 1;
//                 if sets0[h0 as usize].count == 1 {
//                     Q0[Q0_size].index = h0;
//                     Q0[Q0_size].hash = sets0[h0 as usize].xormask;
//                     Q0_size += 1;
//                 }
//                 sets2[h2 as usize].xormask ^= hash;
//                 sets2[h2 as usize].count -= 1;
//                 if sets2[h2 as usize].count == 1 {
//                     Q2[Q2_size].index = h2;
//                     Q2[Q2_size].hash = sets2[h2 as usize].xormask;
//                     Q2_size += 1;
//                 }
//             }
//             while Q2_size > 0 {
//                 Q2_size -= 1;
//                 let key_index_var = Q2[Q2_size];
//                 let index = key_index_var.index as usize;
//                 if sets2[index].count == 0 {
//                     continue;
//                 }
//                 let hash = key_index_var.hash;
//                 let h0 = filter.geth0(hash);
//                 let h1 = filter.geth1(hash);
//                 let mut key_index_var = key_index_var;
//                 key_index_var.index += 2 * filter.block_length;
//                 stack[stack_size] = key_index_var;
//                 stack_size += 1;
//                 sets0[h0 as usize].xormask ^= hash;
//                 sets0[h0 as usize].count -= 1;
//                 if sets0[h0 as usize].count == 1 {
//                     Q0[Q0_size].index = h0;
//                     Q0[Q0_size].hash = sets0[h0 as usize].xormask;
//                     Q0_size += 1;
//                 }
//                 sets1[h1 as usize].xormask ^= hash;
//                 sets1[h1 as usize].count -= 1;
//                 if sets1[h1 as usize].count == 1 {
//                     Q1[Q1_size].index = h1;
//                     Q1[Q1_size].hash = sets1[h1 as usize].xormask;
//                     Q1_size += 1;
//                 }
//             }
//         }
//
//         if stack_size == size as usize {
//             break;
//         }
//
//         if iterations == 10 {
//             let mut keys_vec = Vec::from(keys);
//             keys_vec.sort();
//             keys_vec.dedup();
//             keys = keys_vec.as_slice();
//         }
//
//         reset_sets(&mut sets0);
//         reset_sets(&mut sets1);
//         reset_sets(&mut sets2);
//
//         filter.seed = splitmix64(&mut rng_counter);
//     }
//
//     for stack_size in (0..size).rev() {
//         let ki = stack[stack_size];
//         let mut val = fingerprint(ki.hash) as u8;
//         if ki.index < filter.block_length {
//             let h1 = filter.geth1(ki.hash);
//             let h2 = filter.geth2(ki.hash);
//             let index = ki.index as usize;
//             val ^= filter.fingerprints[h1 as usize + filter.block_length]
//                 ^ filter.fingerprints[h2 as usize + 2 * filter.block_length];
//             filter.fingerprints[index] = val;
//         } else if ki.index < 2 * filter.block_length {
//             let h0 = filter.geth0(ki.hash);
//             let h2 = filter.geth2(ki.hash);
//             let index = ki.index as usize;
//             val ^= filter.fingerprints[h0 as usize]
//                 ^ filter.fingerprints[h2 as usize + 2 * filter.block_length];
//             filter.fingerprints[index] = val;
//         } else {
//             let h0 = filter.geth0(ki.hash);
//             let h1 = filter.geth1(ki.hash);
//             let index = ki.index as usize;
//             val ^= filter.fingerprints[h0 as usize]
//                 ^ filter.fingerprints[h1 as usize + filter.block_length];
//             filter.fingerprints[index] = val;
//         }
//     }
//
//     Ok(filter)
// }
//
// fn prune_duplicates(mut array: Vec<u64>) -> Vec<u64> {
//     array.sort();
//     array.dedup();
//     array
// }
//
// fn main() {
//     let keys = vec![/* your list of keys here */];
//     match populate(&keys) {
//         Ok(filter) => {
//             // Successfully created Xor8 filter
//             // Access filter properties or use it as needed
//         }
//         Err(err) => {
//             eprintln!("Error: {}", err);
//         }
//     }
// }
