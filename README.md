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

## Benchmarks
TODO

## Performance

TODO
## Compliance with the probability of false positives
TODO