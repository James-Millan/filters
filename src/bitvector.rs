/*
We only need to store one bit as 0 or 1 for each element in a filter.
The minimum size in a vector in rust is u8. So each index can store 8 elements of a bloom filter.
 */
#[derive(Debug, Clone)]
pub(crate) struct BitVector {
    pub(crate) array: Vec<u8>,
    pub(crate) size: u64,
}

impl BitVector {
    pub(crate) fn new(size: u64) -> BitVector {
        BitVector   {
            array : BitVector::get_array(size),
            size
        }
    }

    // indexed based on size told to outside world. Need to remember it is 8 times smaller.
    // OR the correct bit with 1. All other digits in the OR'd number should be 0
    pub(crate) fn insert(&mut self, index: u64) {
        let q = index >> 3;
        let r = index & 7;
        self.array[q as usize] |= 1 << r;
    }

    // AND the correct bit with 0. All other digits in the AND'd number should be 1 to not affect things.
    pub(crate) fn delete(&mut self, index: u64)  {
        let q = index >> 3;
        let r = index & 7;
        self.array[q as usize] &= 0b11111111 ^ (1 << r)
    }

    // return true if bit set. false if not.
    pub(crate) fn member(&self, index: u64) -> bool {
        let q = index >> 3;
        let r = index & 7;
        let mask = 1 << r;
        return (self.array[q as usize] & mask)!= 0
    }

    // create array of correct size.
    fn get_array(size : u64) -> Vec<u8>  {
        let mut len= size >> 3;
        let remainder = size  & 7;
        if remainder > 0 {
            len += 1;
        }
        return vec![0; len as usize];
    }

}
