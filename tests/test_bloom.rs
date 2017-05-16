extern crate bloom;

#[cfg(test)]
mod tests {
    use super::bloom::BloomFilter;

    #[test]
    fn test_insert_and_exists() {
        let mut bf = BloomFilter::new(1000, 0.01);
        bf.insert(&"hello");
        assert!(bf.has(&"hello"));
    }

    #[test]
    fn test_insert_and_not_exists() {
        let mut bf = BloomFilter::new(1000, 0.001);
        bf.insert(&"hello");
        bf.insert(&"abcd");
        assert!(!bf.has(&"what"));
    }
}
