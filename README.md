## rust-bloomfilter

Bloom filters are defined by 4 interdependent values:

* n - Number of items in the filter
* p - Probability of false positives, float between 0 and 1 or a number indicating 1-in-p
* m - Number of bits in the filter
* k - Number of hash functions

## Guide for selecting the parameters
The values are interdependent as shown in the following calculations:

```
m = ceil((n * log(p)) / log(1.0 / (pow(2.0, log(2.0)))));

k = round(log(2.0) * m / n);
```
## Design
I use murmur3 hash to generate 128 bit hash integer, and then i split it into two integers of 64 bits each.
Following is the pseudo-code written for the design of bloom filter.

````
let hash_128 = murmur3_hash(data);
let first_64 = (hash_128 & (2_u128.pow(64) - 1));
let second_64 = hash >> 64;
for i 0..num_of_hashfuncs{
    first_64 += i* second_64;
    index =  fist_64 % number_of_bits
    self.bitvec.set(index, true);
}
````
## Usage
````rust
extern crate rust_bloomfilter;

use rust_bloomfilter::BloomFilter;

let mut b = BloomFilter(20000, 0.01, true);
b.add("Helloworld");
assert!(b.contains("Helloworld"));

````
## TODO

1.Adding a constructor for creating a bloomfilter from a BitVec and a number of hash functions
 (so the user can choose how they want to initialize the bloomfilter).If you do that and you
 allow the user to access the bitvec,than they can serialize/deserialize the bloomfilter.
 This way they can create persistent bloomfilters.

2.Consider at least rounding up to whole bytes, as those bits are just wasted otherwise.
But in most circumstances it makes sense to round up to a whole cache line (64 bytes for Intel CPUs).
Rounding up to some sane number of bits lowers your false positive rate and won't increase
the cache footprint of the bloomfilter.

## Benchmarks
TODO

## Performance

TODO
## Compliance with the probability of false positives
TODO