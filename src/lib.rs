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
    dup_check: bool,
}

// m = math.ceil((n * math.log(p)) / math.log(1.0 / (pow(2.0, math.log(2.0)))))
#[inline]
fn nbits(n: usize, p: f64) -> usize {
    let numerator = n as f64 * p.ln();
    let denominator = (1.0_f64 / 2.0_f64.powf(2.0_f64.ln())).ln();
    (numerator / denominator).ceil() as usize
}

// k = round((m / n) * math.log(2));
#[inline]
fn iterations(m: usize, n: usize) -> usize {
    ((m as f64 / n as f64) * 2.0_f64.ln()).round() as usize
}

impl BloomFilter {
    pub fn from_elem(capacity: usize, error_rate: f64, dup_check: bool) -> BloomFilter {
        if capacity == 0 {
            panic!("capacity must be a greater than zero");
        }
        if error_rate <= 0.0 || error_rate > 1.0 {
            panic!("error_rate must be greater than 0.0 and less than 1.0");
        }
        let num_of_bits = nbits(capacity, error_rate);
        let num_of_hashfuncs = iterations(num_of_bits, capacity);
        BloomFilter {
            bitvec: BitVec::from_elem(num_of_bits, false),
            capacity,
            error_rate,
            num_of_hashfuncs,
            num_of_elements: 0,
            dup_check,
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

    pub fn add(&mut self, data: &[u8]) -> Result<bool, &'static str> {
        if self.num_of_elements == self.capacity {
            return Err("You are adding to the bloom filter that is full");
        }
        let hash = hash128(data);
        let hash64_first = (hash & (2_u128.pow(64) - 1)) as u64;
        let hash64_second = (hash >> 64) as u64;
        let mut result_hash: U512 = hash64_first.into();
        let mut exists = true;
        for value in 0..self.num_of_hashfuncs {
            let temp: U512 = U512::from(value) * U512::from(hash64_second);
            result_hash = result_hash.add(temp);
            let index = result_hash % U512::from(self.bitvec_len());
            if self.dup_check && self.bitvec.get(index.as_u64() as usize) == Some(false) {
                exists = false;
            }
            self.bitvec.set(index.as_u64() as usize, true);
        }
        if self.dup_check && exists {
            return Ok(false);
        }
        self.num_of_elements += 1;
        return Ok(true);
    }

    pub fn contains(&self, data: &[u8]) -> bool {
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
        let mut b = BloomFilter::from_elem(20000, 0.01, true);
        assert!(b.add("Test".as_bytes()).unwrap(), true);
        assert!(b.contains("Test".as_bytes()));
    }
    #[test]
    #[should_panic]
    fn test_empty_bloom_zero_capacity_filter() {
        let _b = BloomFilter::from_elem(0, 0.01, true);
    }
    #[test]
    #[should_panic]
    fn test_empty_bloom_zero_error_rate_filter() {
        let _b = BloomFilter::from_elem(10, 0.000, true);
    }

    #[test]
    #[should_panic]
    fn test_empty_bloom_negative_error_rate_filter() {
        let _b = BloomFilter::from_elem(10, -0.010, true);
    }

    #[test]
    fn test_full_bloom_filter() {
        let mut b = BloomFilter::from_elem(10, 0.01, true);
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
        for element in &elements[..9] {
            assert!(b.add(element.as_bytes()).unwrap(), true);
        }
        assert!(
            b.add((&elements[10]).as_ref()).unwrap(),
            "You are adding to the bloom filter that is full"
        );
    }

    #[test]
    fn test_multiple_elements() {
        let mut b = BloomFilter::from_elem(20000, 0.01, true);
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
            b.add(element.as_bytes()).unwrap();
        }
        for element in &elements {
            assert!(b.contains(element.as_bytes()));
        }
        assert_eq!(b.contains("rajaa".as_bytes()), false);
        assert_eq!(elements.len(), b.len())
    }
    #[test]
    fn test_multiple_duplicate_elements() {
        let mut b = BloomFilter::from_elem(20000, 0.01, true);
        let elements = vec!["Srinivas", "Srinivas", "Reddy", "Reddy"];
        assert_eq!(b.add(elements[0].as_bytes()).unwrap(), true);
        assert_eq!(b.len(), 1);
        assert_eq!(b.add(elements[1].as_bytes()).unwrap(), false);
        assert_eq!(b.len(), 1);
        assert_eq!(b.add(elements[2].as_bytes()).unwrap(), true);
        assert_eq!(b.len(), 2);
        assert_eq!(b.add(elements[3].as_bytes()).unwrap(), false);
        assert_eq!(b.len(), 2);
    }

    #[test]
    fn test_multiple_duplicate_elements_with_dup_check_false() {
        let mut b = BloomFilter::from_elem(20000, 0.01, false);
        let elements = vec!["Srinivas", "Srinivas", "Reddy", "Reddy"];
        assert_eq!(b.add(elements[0].as_bytes()).unwrap(), true);
        assert_eq!(b.len(), 1);
        assert_eq!(b.add(elements[1].as_bytes()).unwrap(), true);
        assert_eq!(b.len(), 2);
        assert_eq!(b.add(elements[2].as_bytes()).unwrap(), true);
        assert_eq!(b.len(), 3);
        assert_eq!(b.add(elements[3].as_bytes()).unwrap(), true);
        assert_eq!(b.len(), 4);
        for i in vec!["Srinivas", "Reddy"] {
            assert!(b.contains(i.as_bytes()))
        }
    }
}
