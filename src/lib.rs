extern crate bigint;
extern crate bit_vec;
extern crate fasthash;

use bigint::uint::U512;
use bit_vec::BitVec;
use fasthash::murmur3::hash128;
use std::ops::Add;

pub struct BloomFilter {
    capacity: usize,
    bitvec: BitVec,
    error_rate: f64,
    num_of_hashfuncs: usize,
    num_of_elements: usize,
}

// m = math.ceil((n * math.log(p)) / math.log(1.0 / (pow(2.0, math.log(2.0)))))

fn nbits(n: usize, p: f64) -> usize {
    let numerator = n as f64 * p.ln();
    let denominator = (1.0_f64 / 2.0_f64.powf(2.0_f64.ln())).ln();
    (numerator / denominator).ceil() as usize
}

// k = round((m / n) * math.log(2));

fn iterations(m: usize, n: usize) -> usize {
    ((m as f64 / n as f64) * 2.0_f64.ln()).round() as usize
}

impl BloomFilter {
    pub fn from_elem(capacity: usize, error_rate: f64) -> BloomFilter {
        if capacity == 0 {
            panic!("capacity must be a greater than zero");
        }
        if error_rate == 0.0 {
            panic!("error_rate must be greater than zero");
        }
        let num_of_bits = nbits(capacity, error_rate);
        let num_of_hashfuncs = iterations(num_of_bits, capacity);
        BloomFilter {
            bitvec: BitVec::from_elem(num_of_bits, false),
            capacity,
            error_rate,
            num_of_hashfuncs,
            num_of_elements: 0,
        }
    }

    pub fn bitvec_len(&self) -> usize {
        self.bitvec.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.num_of_elements
    }

    pub fn error_rate(&self) -> f64 {
        self.error_rate
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, data: &str) {
        if self.num_of_elements == self.capacity {
            panic!("You are adding to the bloom filter that is full");
        }
        let hash = hash128(data);
        let hash64_first = (hash & (2_u128.pow(64) - 1)) as u64;
        let hash64_second = (hash >> 64) as u64;
        let mut result_hash: U512 = hash64_first.into();
        for value in 0..self.num_of_hashfuncs {
            let temp: U512 = U512::from(value) * U512::from(hash64_second);
            result_hash = result_hash.add(temp);
            let index = result_hash % U512::from(self.bitvec_len());
            self.bitvec.set(index.as_u64() as usize, true);
        }
        self.num_of_elements += 1;
    }

    pub fn contains(&self, data: &str) -> bool {
        let hash = hash128(data);
        let hash64_first = (hash & (2_u128.pow(64) - 1)) as u64;
        let hash64_second = (hash >> 64) as u64;
        let mut result_hash: U512 = hash64_first.into();
        for value in 0..self.num_of_hashfuncs {
            let temp: U512 = U512::from(value) * U512::from(hash64_second);
            result_hash = result_hash.add(temp);
            let index = result_hash % U512::from(self.bitvec_len());
            if self.bitvec.get(index.as_u64() as usize) == Some(false) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::BloomFilter;

    #[test]
    fn test_single_element() {
        let mut b = BloomFilter::from_elem(20000, 0.01);
        b.add("Test");
        assert!(b.contains("Test"));
    }
    #[test]
    #[should_panic]
    fn test_empty_bloom_zero_capacity_filter() {
        let _b = BloomFilter::from_elem(0, 0.01);
    }
    #[test]
    #[should_panic]
    fn test_empty_bloom_zero_error_rate_filter() {
        let _b = BloomFilter::from_elem(10, 0.000);
    }

    #[test]
    #[should_panic]
    fn test_full_bloom_filter() {
        let mut b = BloomFilter::from_elem(10, 0.01);
        // Add 11 elements to the 10 capacity Bloomfilter
        let elements = vec![
            "Srinivas",
            "Reddy",
            "Gundrapally",
            "Nekkonda",
            "Warangal",
            "Telangana",
            "506122",
            "Srinivas1",
            "Reddy1",
            "Gundrapally1",
            "Telangana1",
        ];
        for element in &elements {
            b.add(element);
        }
    }

    #[test]
    fn test_multiple_element() {
        let mut b = BloomFilter::from_elem(20000, 0.01);
        let elements = vec![
            "Srinivas",
            "Reddy",
            "Gundrapally",
            "Nekkonda",
            "Warangal",
            "Telangana",
            "506122",
        ];
        for element in &elements {
            b.add(element);
        }
        for element in &elements {
            assert!(b.contains(element));
        }
        assert_eq!(b.contains("rajaa"), false);
        assert_eq!(elements.len(), b.len())
    }
}
