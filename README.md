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
Following is pseudo-code written for the design of bloom filter.

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
## Benchmarks
TODO

## Performance

TODO
## Compliance with the probability of false positives
TODO