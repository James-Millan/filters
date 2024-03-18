use crate::bitvector::BitVector;
pub struct MortonBlock {
    pub(crate) fsa: Vec<u8>,
    pub(crate) fca: BitVector,
    pub(crate) ota: BitVector
}

impl MortonBlock {
    pub(crate) fn new() -> MortonBlock {
        return MortonBlock {
            fsa: vec![0; 48],
            fca: BitVector::new(128),
            ota: BitVector::new(32),
        }
    }
}