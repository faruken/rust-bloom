[![Build Status](https://travis-ci.org/faruken/rust-bloom.svg?branch=develop)](https://travis-ci.org/faruken/rust-bloom)
![](https://img.shields.io/badge/rustc-1.19.0--nightly-lightgrey.svg)
![](https://img.shields.io/badge/License-MIT-blue.svg)

# rust-bloom

rust-bloom is a bloom filter implementation in rust that uses farmhash.


### Example Usage

    extern crate bloom;

    use bloom::BloomFilter;

    fn main() {
        let mut bf = BloomFilter::new(10_000, 0.02);
        bf.insert(&"hello");
        bf.insert(&"abcd");
        
        match bf.has(&"hello") {
            true => println!("exists"),
            _ => println!("not exists"),
        }
    }


