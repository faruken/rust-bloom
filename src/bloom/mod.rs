extern crate bit_vec;
extern crate farmhash;

use std::default::Default;
use std::f64::consts;
use bit_vec::BitVec;
use farmhash::hash64;


pub struct BloomFilter {
    bv: BitVec,
    hashes: usize
}


impl Default for BloomFilter {
    #[inline]
    fn default() -> BloomFilter {
        // http://hur.st/bloomfilter?n=10000&p=0.001
        BloomFilter::new(10_000, 0.001)
    }
}


impl BloomFilter {
    pub fn new(capacity: usize, error_rate: f64) -> BloomFilter {
        assert!((error_rate > 0.0 && error_rate < 1.0) && capacity > 0);

        // https://en.wikipedia.org/wiki/Bloom_filter#Probability_of_false_positives
        let m = BloomFilter::num_of_bits_in_vec(capacity, error_rate);

        // https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions
        let k = BloomFilter::num_of_hash_funcs(m, capacity);

        BloomFilter {
            bv: BitVec::from_elem(capacity, false),
            hashes: k
        }
    }

    fn num_of_bits_in_vec(capacity: usize, error_rate: f64) -> usize {
        (-1.0 * (((capacity as f64) * error_rate.ln()) / (1.0 / consts::LN_2.powf(2.0)).ln())).ceil() as usize
    }

    fn num_of_hash_funcs(m: usize, capacity: usize) -> usize {
        (consts::LN_2 * ((m as f64) / (capacity as f64))).round().abs() as usize
    }

    pub fn insert(&mut self, value: &str) {
        for i in 0..self.hashes {
            let index = self.nth_hash(&value, i);
            self.bv.set(index, true);
        }
    }

    fn nth_hash(&self, x: &str, m: usize) -> usize {
        let hashval = x.to_string() + &m.to_string(); // meh
        (hash64(&hashval.as_bytes()) % (self.bv.capacity() as u64)) as usize
    }

    pub fn has(&self, value: &str) -> bool {
        for i in 0..self.hashes {
            let index = self.nth_hash(&value, i);
            if !self.bv[index] {
                return false;
            }
        }
        true
    }
}
