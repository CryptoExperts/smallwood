# hash

This crate provides a minimal implementation of several algebraic cryptographic hash functions over a 64-bit prime field,
aimed at providing benchmarking comparison of those instances with Anemoi.

For each of those contenders, two instantiations are being tested:

- one of width 12 and rate 8, at 128 bits of security, with regular sponge construction for compression
- one of width 8 and rate 4, at 128 bits of security, with Jive for compression

## Benchmarking

To run benchmarks, execute the following command in a terminal:

```
RUSTFLAGS=-Ctarget-cpu=native cargo bench
```

To run benchmarks for a single type of hash function, for instance Poseidon, run:

```
RUSTFLAGS=-Ctarget-cpu=native cargo bench --bench poseidon
```

Below are running times obtained on an Intel® Core™ i7-9750H CPU @ 2.60GHz × 12 running Ubuntu 22.04 LTS: (RP stands for Rescue-Prime)

| Running time       | Anemoi 8-4 | Griffin 8-4 | Griffin 12-8 | RP 8-4  | RP 12-8  | Poseidon 8-4 | Poseidon 12-8 |
| ------------------ | :--------: | :---------: | :----------: | :-----: | :------: | :----------: | :-----------: |
| 2-to-1 compression | 4.21 µs    | 2.59 µs     | 2.87 µs      | 9.13 µs | 15.67 µs | 2.69 µs      | 5.87 µs      |

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
