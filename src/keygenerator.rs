use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use rand::prelude::SliceRandom;
use rand::Rng;

pub struct KeyGenerator {
    pub random: (Vec<u64>, Vec<u64>),
    pub disjoint: (Vec<u64>, Vec<u64>),
    pub mixed: (Vec<u64>, Vec<u64>)
}

impl KeyGenerator {
    pub fn new(size:u64) -> KeyGenerator {
        return KeyGenerator {
            random: Self::generate_random_keys(size),
            disjoint: Self::generate_disjoint_keys(size),
            mixed: Self::generate_mixed_keys(size),
        }
    }
    pub fn new_empty() -> KeyGenerator {
        return KeyGenerator {
            random: (vec![], vec![]),
            disjoint: (vec![], vec![]),
            mixed: (vec![], vec![]),
        }
    }
    fn generate_random_keys(size: u64) -> (Vec<u64>, Vec<u64>) {
        let mut set_keys = HashSet::new();
        let mut rng = rand::thread_rng();

        while set_keys.len() < size as usize {
            let random_value: u64 = rng.gen_range(0..(size as f64*2.5f64) as u64);
            if !set_keys.contains(&random_value) {
                set_keys.insert(random_value);
            }
        }
        let keys: Vec<u64> = set_keys.iter().copied().collect();
        
        let mut lookup_keys = HashSet::new();
        while lookup_keys.len() < size as usize {
            let random_value: u64 = rng.gen_range(0..(size as f64*2.5f64) as u64);
            if !set_keys.contains(&random_value) && !lookup_keys.contains(&random_value) {
                lookup_keys.insert(random_value);
            }
        }

        let lookup_keys: Vec<u64>  = lookup_keys.iter().copied().collect();
        return (keys,lookup_keys)
    }

    fn generate_mixed_keys(size:u64) -> (Vec<u64>, Vec<u64>) {
        let mut rng = rand::thread_rng();

        let mut set_keys = HashSet::new();
        while set_keys.len() < size as usize {
            let random_value: u64 = rng.gen_range(0..(size as f64*2.5f64) as u64);
            if !set_keys.contains(&random_value) {
                set_keys.insert(random_value);
            }
        }

        let mut list: Vec<u64> = set_keys.iter().copied().collect();


        list.shuffle(&mut rng);


        let mut queries: Vec<u64> = list.iter().take(list.len()/2).copied().collect();

        while queries.len() < set_keys.len() {
            let random_value: u64 = rng.gen_range(0..(size as f64*2.5f64) as u64);
            if !set_keys.contains(&random_value) {
                queries.push(random_value);
            }
        }
        return (list,queries)
    }
    fn generate_disjoint_keys(size:u64) -> (Vec<u64>, Vec<u64>) {
        return ((0..size).collect(),(size..2*size).collect());
    }

    pub fn write_to_file(&self) -> Result<(), std::io::Error> {
        let file = File::create("random_keys")?;
        let mut writer = BufWriter::new(file);
        for num in &self.random.0 {
            write!(writer, "{} ", num)?;
        }
        writeln!(writer)?;
        for num in &self.random.1 {
            write!(writer, "{} ", num)?;
        }

        let file = File::create("disjoint_keys")?;
        let mut writer = BufWriter::new(file);
        for num in &self.disjoint.0 {
            write!(writer, "{} ", num)?;
        }
        writeln!(writer)?;
        for num in &self.disjoint.1 {
            write!(writer, "{} ", num)?;
        }

        let file = File::create("mixed_keys")?;
        let mut writer = BufWriter::new(file);
        for num in &self.mixed.0 {
            write!(writer, "{} ", num)?;
        }
        writeln!(writer)?;
        for num in &self.mixed.1 {
            write!(writer, "{} ", num)?;
        }
        return Ok(());
    }
    pub fn read_from_file(&mut self) -> Result<(), std::io::Error> {
        let file = File::open("random_keys")?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        if let (Some(first_line), Some(second_line)) = (lines.next(), lines.next()) {
            let first_vec: Vec<u64> = first_line?
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let second_vec: Vec<u64> = second_line?
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            self.random = (first_vec,second_vec);
        }
        let file = File::open("disjoint_keys")?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        if let (Some(first_line), Some(second_line)) = (lines.next(), lines.next()) {
            let first_vec: Vec<u64> = first_line?
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let second_vec: Vec<u64> = second_line?
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            self.disjoint = (first_vec,second_vec);
        }

        let file = File::open("mixed_keys")?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        if let (Some(first_line), Some(second_line)) = (lines.next(), lines.next()) {
            let first_vec: Vec<u64> = first_line?
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let second_vec: Vec<u64> = second_line?
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            self.mixed = (first_vec,second_vec);
        }
        return Ok(());
    }
}