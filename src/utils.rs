
// x is key to be hashed. l is binary log of filter size. a1,a2,b random u64s.
pub(crate) fn hash(x: u64, l: u32, a1: u64, a2: u64, b: u64) -> u32 {
    //return (((a1 + x) * (a2 + (x >> 32)) + b) >> (64 - l)) as usize
    return ((a1.wrapping_add(x)).wrapping_mul(a2.wrapping_add((x >> 32))).wrapping_add(b) >> (64 - l)) as u32;
}

// logarithms with a chose base.
pub(crate) fn log_base(x: f64, base: f64) -> f64 {
    return x.ln() / base.ln()
}

