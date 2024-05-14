#[path = "bitvector.rs"]
mod bitvector;

use bitvector::BitVector;
#[derive(Debug)]
pub struct MortonBlock {
    pub(crate) fsa: Vec<u8>,
    pub(crate) fca: Vec<u8>,
    pub(crate) ota: Vec<u8>,
}

impl MortonBlock {
    pub(crate) fn new() -> MortonBlock {
        return MortonBlock {
            fsa: vec![0; 48],
            fca: vec![0;64],
            ota: vec![0;16],
        }
    }
}