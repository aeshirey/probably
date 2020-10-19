# Approximate computing
[Approximate computing](https://en.wikipedia.org/wiki/Approximate_computing) "is a computation technique which returns a possibly inaccurate result rather than a guaranteed accurate result," which "can provide disproportionate gains in performance and energy, while still achieving acceptable result accuracy."

This library is a collection of several approximate computing algorithms written in Rust. The goal is to provide excellent performance for the majority of use-cases. When possible, algorithms should be capable of being parallelized (ie, map-reduce of the core algorithm) and serialized.

**NOTE** that I did not write any of these algorithms. They are all implemented by other talented Rustaceans, the repositories for which are linked below. All source code is MIT- or Apache-licensed.

The algorithms implemented in this library and on the roadmap fall into one of several categories:

## Cardinality/frequency
These algorithms approxmate counting the number of items in a set within some approximate acceptable error.

- **HyperLogLog (HLL)** -- implemented from [`rust-hyperloglog`](https://github.com/jedisct1/rust-hyperloglog)
- Count-min sketch to serve as a frequency table of events

## Membership
These algorithms determine if an item _n_ exists in the set _N_, guaranteeing no false negatives at the cost of some false positives.

- Bloom Filter
- Quotient filter - like Bloom filter, but can be merged and resized more efficiently. Uses 10-25% more space than BF.
   - [MQF](https://www.biorxiv.org/content/10.1101/2020.08.23.263061v1)
- Cuckoo filter - like Bloom filter, but can delete items. Can use lower space overhead than BF.


# Other
- MinHash for estimating similarity of two sets