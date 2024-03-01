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
        for _ in 0..20 {
            let mut lookup = vec![];
            let mut rng = rand::thread_rng();
            for _ in 0..10 {
                let random_value: u64 = rng.gen_range(0..u64::MAX);
                lookup.push(random_value);
            }
            lookups.push(lookup);
        }
        return lookups
    }

    fn hash(&self, x: u64, i: usize) -> u64 {
        return self.lookups[i][x as usize];
    }

    pub(crate) fn tabulation_hashing(&self, x: u64) -> u64 {

        // obtain digits from key.
        let mut x = x;
        let mut digits = Vec::new();

        if x == 0 {
            digits.push(0);
        } else {
            while x > 0 {
                digits.push(x % 10);
                x /= 10;
            }
        }
        // now hash and xor all digits together
        let mut res = 0;
        for i in 0..digits.len() {
            res ^= self.hash(digits[i], i);
        }
        return res;
    }
}

