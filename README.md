# Approximate computing
[Approximate computing](https://en.wikipedia.org/wiki/Approximate_computing) "is a computation technique which returns a possibly inaccurate result rather than a guaranteed accurate result," which "can provide disproportionate gains in performance and energy, while still achieving acceptable result accuracy."

This library is a collection of several approximate computing algorithms written in Rust. The goals are:
* **Ease of use**. For maximum adoption, it should be straightforward to make use of AC algorithms with very little effort.
* **Parallelization**. When possible, algorithms should be capable of being parallelized (ie, map-reduce of the core algorithm). This includes serialization of datasets.
* **Common dependencies**. Minimize the dependency chain to reduce build times and binary sizes.

**NOTE** that I did not write any of these algorithms - they are all implemented by other talented Rustaceans, the repositories for which are linked below. I have, however, added some functionality (serialization/deserialization, exposing new initialization functions, etc.), and I did implement the Rust version from C#. All source code is MIT-licensed.

## Using
To use `probably` in your Cargo.toml:

```toml
probably = "0.0.1"
```

Note that as of 2020-11-25, the P2 quantile estimator is not yet included in the published crate.

To include serialization of data structures, include the `with_serde` feature:

```toml
probably = { version = "0.0.1", features = ["with_serde"] }
```

The algorithms implemented in this library and on the roadmap fall into one of several categories:

## Cardinality/frequency
These algorithms approxmate counting the number of items in a set within some approximate acceptable error.

- [X] **HyperLogLog (HLL)** -- implemented from [`rust-hyperloglog`](https://github.com/jedisct1/rust-hyperloglog)
- [X] **Count-min sketch** -- implemented from [`rust-count-min-sketch`](https://github.com/jedisct1/rust-count-min-sketch/) to serve as a frequency table of events

## Membership
These algorithms determine if an item _n_ exists in the set _N_, guaranteeing no false negatives at the cost of some false positives.

- [X] **Bloom Filter** -- implemented from [`bloom_filter`](https://github.com/jeromefroe/bloom_filter)
- [ ] **Quotient filter** -- like Bloom filter, but can be merged and resized more efficiently. Uses 10-25% more space than BF.
   - [Paper on MQF](https://www.biorxiv.org/content/10.1101/2020.08.23.263061v1), "a compact hashtable, can efficiently store k-mers with a skewed distribution"
   - [MQF implemented in C++](https://github.com/dib-lab/MQF)
- [X] **Cuckoo filter** -- implemented from [`rust-cuckoofilter`](https://github.com/axiomhq/rust-cuckoofilter). like Bloom filter, but can delete items. Can use lower space overhead than BF.

## Quantile Estimators
- [X] **[Piecewise-Parabolic Quantile Estimator](https://aakinshin.net/posts/p2-quantile-estimator/)** -- translated from [`perfolizer` C#](https://github.com/AndreyAkinshin/perfolizer/blob/f5615525ce36140d13bf6cf9fdf98f48c3e23206/src/Perfolizer/Perfolizer/Mathematics/QuantileEstimators/P2QuantileEstimator.cs)
   - [Other `perfolizer` estimators](https://github.com/AndreyAkinshin/perfolizer/tree/f5615525ce36140d13bf6cf9fdf98f48c3e23206/src/Perfolizer/Perfolizer/Mathematics/QuantileEstimators)
- [X] **[Greenwald-Khanna](http://infolab.stanford.edu/~datar/courses/cs361a/papers/quantiles.pdf)** -- implemented from [`postmates/quantiles`](https://github.com/postmates/quantiles)
- [X] **[Misra-Gries](https://people.csail.mit.edu/rrw/6.045-2017/encalgs-mg.pdf)** -- also from `postmates/quantiles`
- [X] **Histogram** -- also from `postmates/quantiles`

## Other
- [ ] MinHash for estimating similarity of two sets

# Additional reading
* [Probabilistic Data Structures](https://iq.opengenus.org/probabilistic-data-structures/)
