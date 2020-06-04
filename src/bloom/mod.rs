extern crate farmhash;

use std::default::Default;
use farmhash::FarmHasher;
use std::hash::{Hash, Hasher};


struct BloomFilter {
    bv: Vec<bool>,
    hashes: u64,
}


impl Default for BloomFilter {
    #[inline]
    fn default() -> BloomFilter {
        // http://hur.st/bloomfilter?n=10000&p=0.001
        BloomFilter::new(10_000, 0.001)
    }
}


impl BloomFilter {
    pub fn new(capacity: usize, error_rate: f64) -> Self {
        assert!((error_rate > 0.0 && error_rate < 1.0) && capacity > 0);
        let bv = vec![false; capacity];

        // https://en.wikipedia.org/wiki/Bloom_filter#Probability_of_false_positives
        let m = BloomFilter::num_of_bits_in_vec(capacity, error_rate);

        // https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions
        let k = BloomFilter::num_of_hash_funcs(m, capacity);

        BloomFilter {
            bv,
            hashes: k,
        }
    }
    
    #[inline]
    fn num_of_bits_in_vec(capacity: usize, error_rate: f64) -> usize {
        (-1.0 * (((capacity as f64) * error_rate.ln()) /
            (1.0 / std::f64::consts::LN_2.powf(2.0)).ln())).ceil() as usize
    }

    #[inline]
    fn num_of_hash_funcs(m: usize, capacity: usize) -> u64 {
        (std::f64::consts::LN_2 * ((m as f64) / (capacity as f64))).round().abs() as u64
    }

    fn nth_hash<T>(&self, x: T, m: u64) -> usize where T: Hash {
        let mut hasher = FarmHasher::default();
        hasher.write(&m.to_be_bytes());
        x.hash(&mut hasher);
        ((hasher.finish()) % (self.bv.capacity() as u64)) as usize
    }

    pub fn insert<T>(&mut self, value: T) -> bool where T: Hash {
        for i in 0..self.hashes {
            let index = self.nth_hash(&value, i);
            self.bv[index] = true;
        }
        true
    }

    pub fn has<T>(&self, value: T) -> bool where T: Hash {
        for i in 0..self.hashes {
            let index = self.nth_hash(&value, i);
            if !self.bv[index] {
                return false;
            }
        }
        true
    }
}
