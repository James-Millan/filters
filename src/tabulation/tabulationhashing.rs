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
        for _ in 0..8 {
            let mut lookup = vec![];
            for _ in 0..256 {
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
        res ^= self.hash(x as u8 & 0x20, 0);
        res ^= self.hash((x >> 8) as u8 & 0x20, 1);
        res ^= self.hash((x >> 16) as u8 & 0x20, 2);
        res ^= self.hash((x >> 24) as u8 & 0x20, 3);
        res ^= self.hash((x >> 32) as u8 & 0x20, 4);
        res ^= self.hash((x >> 40) as u8 & 0x20, 5);
        res ^= self.hash((x >> 48) as u8 & 0x20, 6);
        res ^= self.hash((x >> 56) as u8 & 0x20, 7);
        // res ^= self.hash((x >> 60) as u8 & 0x20, 8);
        // res ^= self.hash((x >> 64) as u8 & 0x0F, 16);
        // for i in 0..16 {
        //     let digit = (x >> (i * 4)) as u8 & 0x0F;
        //     // println!("{}", digit);
        //
        // }
        return res;
    }
}