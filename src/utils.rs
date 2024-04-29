use std::collections::VecDeque;
use std::hash::Hash;
use rand::Rng;

struct HashInfo {
    l: u32,
    hashes: Vec<(u64,u64,u64)>,
    size: u64
}


pub(crate) fn map(x:u64, n:u64) -> u64 {
    return x % n;
}

pub(crate) fn max(x:f64, y: f64) -> f64 {
    if x >= y {
        return x;
    }
    else {
        return y;
    }
}

// x is key to be hashed. l is binary log of filter size. a1,a2,b random u64s.
pub(crate) fn hash(x: u64, l: u32, a1: u64, a2: u64, b: u64) -> u32 {
    //return (((a1 + x) * (a2 + (x >> 32)) + b) >> (64 - l)) as usize
    return ((a1.wrapping_add(x)).wrapping_mul(a2.wrapping_add(x >> 32)).wrapping_add(b) >> (64 - l)) as u32;
}

// logarithms with a chose base.
pub(crate) fn log_base(x: f64, base: f64) -> f64 {
    return x.ln() / base.ln()
}

pub(crate) fn perfect_hashing(keys: &Vec<u64>) -> Vec<u32> {
    return construct(keys);
}

pub(crate) fn perfect_hashing_member(ranktable: Vec<u32>) -> bool {
    return true;
}

fn construct(keys: &Vec<u64>) -> Vec<u32> {
    let size = ((1.23 * keys.len() as f64).floor() + 32.0) as u64;
    let mut finished = false;
    let l = log_base(size as f64, 2f64) as u32;
    let mut hash_info = HashInfo {
        l,
        hashes: Vec::new(),
        size
    };
    let mut sigma = Vec::new();
    while !finished {
        let mut rng = rand::thread_rng();
        let mut hash_functions = Vec::new();

        for _ in 0..=2 {
            let a1: u64 = rng.gen_range(1..=u64::MAX );
            let a2: u64 = rng.gen_range(1..=u64::MAX);
            let b: u64 = rng.gen_range(1..=u64::MAX);
            hash_functions.push((a1,a2,b));
        }
        hash_info.hashes = hash_functions;
        (finished, sigma) = mapping(&keys, &hash_info);
    }
    return assign(&sigma, &hash_info);
}
fn mapping(keys: &Vec<u64>, hash_info: &HashInfo) -> (bool, Vec<(u64, usize)>) {
    let c = hash_info.size;

    let mut h: Vec<(u64,usize)> = vec![(0,0); c as usize];
    for i in 0..keys.len() {
        let x = keys[i];
        let h0 = hash0(x, &hash_info);
        let h1 = hash1(x, &hash_info);
        let h2 = hash2(x, &hash_info);
        h[h0 as usize] = ((x ^ h[h0 as usize].0), h[h0 as usize].1 + 1);
        h[h1 as usize] = ((x ^ h[h1 as usize].0), h[h1 as usize].1 + 1);
        h[h2 as usize] = ((x ^ h[h2 as usize].0), h[h2 as usize].1 + 1);
    }
    let mut q = VecDeque::new();
    let mut sigma = Vec::new();
    for i in 0..h.len() {
        if h[i].1 == 1 {
            q.push_back(i);
        }
    }
    while !q.is_empty() {
        let i = q.pop_front().unwrap();
        if h[i].1 == 1 {
            let x = h[i].0;
            // needs to be a stack.
            sigma.push((x, i));
            let h0 = hash0(x, &hash_info);
            let h1 = hash1(x, &hash_info);
            let h2 = hash2(x, &hash_info);
            // remove x from h[h_j]
            h[h0 as usize] = ((x ^ h[h0 as usize].0), h[h0 as usize].1 - 1);
            h[h1 as usize] = ((x ^ h[h1 as usize].0), h[h1 as usize].1 - 1);
            h[h2 as usize] = ((x ^ h[h2 as usize].0), h[h2 as usize].1 - 1);
            if h[h0 as usize].1 == 1 {
                q.push_back(h0 as usize);
            }
            if h[h1 as usize].1 == 1 {
                q.push_back(h1 as usize);
            }
            if h[h2 as usize].1 == 1 {
                q.push_back(h2 as usize);
            }
        }
    }
    if sigma.len() == keys.len() {
        //println!("'{}'", sigma.len());
        return (true, sigma);
    }
    else {
        //println!("'{:?}'", sigma);
        //println!("'{}'", sigma.len());
        return (false, (Vec::new()));
    }
}
fn assign(sigma: &Vec<(u64, usize)>, hash_info: &HashInfo) -> Vec<u32> {
    let c= hash_info.size;
    let size = sigma.len();
    let mut b = vec![3u32; c as usize ];
    for j in (0..size).rev() {
        let (x,i) = sigma[j];
        b[i] = p(x, &b, hash_info);
    }
    return b;
}

fn ranking(keys: &Vec<u64>, b: &Vec<u32>) -> Vec<u32> {
    let mut ranks = Vec::new();
    ranks.push(0);
    for i in 1..keys.len() {
        let mut rank = ranks[i-1];
        if b[i-1] != 3 {
            rank += 1;
        }
        ranks[i] = rank
    }
    return ranks;
}

fn rank(b: &Vec<u32>) -> Vec<u32> {
    let ranks = Vec::new();

    return ranks;
}

fn p(x: u64, b: &Vec<u32>, hash_info: &HashInfo) -> u32 {
    let i = (b[hash0(x, hash_info) as usize] + b[hash1(x,hash_info) as usize] + b[hash2(x,hash_info) as usize]) % 3;
    if i == 0 {
        return hash0(x, hash_info);
    }
    else if i == 1 {
        return hash1(x, hash_info);
    }
    else {
        return hash2(x,hash_info);
    }
}

pub(crate) fn fingerprint(key: u64, hash_info: &HashInfo) -> u32 {
    return hash(key,hash_info.l, hash_info.hashes[0].0, hash_info.hashes[0].1, hash_info.hashes[0].2) as u32;
}
fn hash0(key: u64, hash_info: &HashInfo) -> u32 {
    let bound = hash_info.size / 3;
    let res = hash(key,hash_info.l, hash_info.hashes[0].0, hash_info.hashes[0].1, hash_info.hashes[0].2) % bound as u32;
    //println!("'{}','{}'",0, res);
    return res;

}
fn hash1(key: u64, hash_info: &HashInfo) -> u32 {
    let bound = hash_info.size / 3;
    let mut res = hash(key,hash_info.l, hash_info.hashes[1].0, hash_info.hashes[1].1, hash_info.hashes[1].2) % bound as u32;
    res = (bound + res as u64) as u32;
    //println!("'{}','{}'",1, res);
    return res
}
fn hash2(key: u64, hash_info: &HashInfo) -> u32 {
    let bound = hash_info.size / 3;
    let mut res = (hash(key, hash_info.l, hash_info.hashes[2].0, hash_info.hashes[2].1, hash_info.hashes[2].2)) % bound as u32;
    res = ((2 * bound) + res as u64) as u32;
    //println!("'{}','{}'",2, res);
    return res;
}

pub(crate) fn closest_power_of_two(n: u64) -> u64 {
    if n <= 0 {
        return 0;
    }
    let exponent = (n as f64).log2().round();
    return 2u64.pow(exponent as u32);
}


