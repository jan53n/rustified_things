use bit_array::BitArray;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

mod bit_array;

struct BloomFilter {
    hash_count: usize,
    size: usize,
    store: BitArray,
}

impl BloomFilter {
    fn new(fp_prob: f64, item_count: usize) -> Self {
        let size = Self::get_size(item_count, fp_prob) as usize;
        let hash_count = Self::get_hash_count(size, item_count) as usize;

        Self {
            hash_count,
            size,
            store: BitArray::new(size),
        }
    }

    fn insert<H: Hash>(&mut self, item: H) -> bool {

        for i in 0..self.hash_count {
            let mut s = DefaultHasher::new();
            item.hash(&mut s);
            s.write_usize(i);
            self.store.flip((s.finish() % self.size as u64) as usize);
        }

        true
    }

    fn check<H: Hash>(&mut self, item: H) -> bool {

        for i in 0..self.hash_count {
            let mut s = DefaultHasher::new();
            item.hash(&mut s);
            s.write_usize(i);
            
            if self.store.get((s.finish() % self.size as u64) as usize) == false {
                return false;
            }
        }

        true
    }

    // math magic is beyond me!
    fn get_size(n: usize, p: f64) -> f64 {
        -(n as f64 * p.log2())/(0.4804530139182014)
    }

    // math magic is beyond me!
    fn get_hash_count(m: usize, n: usize) -> f64 {
        (m/n) as f64 * 0.6931471805599453
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculate_bit_array_size() {
        assert_eq!(BloomFilter::get_size(10, 0.1), 69.1415809382961 as f64);
    }

    #[test]
    fn it_insert_string() {
        let mut bf = BloomFilter::new(0.1, 1000);
        assert_eq!(bf.insert("item: H"), true);
    }

    #[test]
    fn it_check_string() {
        let mut bf = BloomFilter::new(0.1, 1000);
        bf.insert("item: H");
        assert_eq!(bf.check("item: H"), true);
    }

    #[test]
    fn it_should_fail_on_check_string() {
        let mut bf = BloomFilter::new(0.1, 1000);
        bf.insert("item: H");
        assert_eq!(bf.check("i'm not who you think i am"), false);
    }
}
