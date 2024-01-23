

pub(crate) struct XorFilter {
    pub(crate) bit_array : Vec<bool>,
    hash_functions : Vec<(u64,u64,u64)>,
    size: u64,
    l: u64
}
