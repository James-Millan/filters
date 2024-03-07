use rand::Rng;

pub(crate) struct TabulationHashing {
    lookups: Vec<Vec<u64>>
}
impl TabulationHashing {
    pub(crate) fn new() -> Self {
        let lookups = Self::generate_lookups();
        TabulationHashing {
            lookups
        }
    }

    fn generate_lookups() -> Vec<Vec<u64>> {
        let mut lookups = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..16 {
            let mut lookup = vec![];
            for _ in 0..16 {
                let random_value: u64 = rng.gen_range(0..u64::MAX);
                lookup.push(random_value);
            }
            lookups.push(lookup);
        }
        return lookups
    }

    fn hash(&self, x: u8, i: usize) -> u64 {
        return self.lookups[i][x as usize];
    }

    pub(crate) fn tabulation_hashing(&self, x: u64) -> u64 {
        // obtain hex digits from key and xor lookups together.
        let mut res = 0;
        for i in 0..16 {
            let digit = (x >> (i * 4)) as u8 & 0x0F;
            // println!("{}", digit);
            res ^= self.hash(digit, i);
        }
        return res;
    }
}