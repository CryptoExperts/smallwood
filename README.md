# SmallWood Proof System & CAPSS Framework

This repository contains a prototype implementation (in C) of the **SmallWood** proof system and the **CAPSS** framework.

- **SmallWood** is a hash-based proof system targeting small-to-medium statements.
- **CAPSS** is a framework for constructing SNARK-friendly, post-quantum signature schemes from arithmetization-oriented permutations.


📄 The accompanying paper introducing SmallWood and CAPSS is available on the IACR ePrint archive: [https://eprint.iacr.org/2025/061](https://eprint.iacr.org/2025/061).

⚠️ **Warning**: This is a prototype implementation. It has not been audited and may contain bugs or security vulnerabilities. It is **not** intended for production use.

## 🚧 Dependencies

CAPSS relies on external implementations of the underlying arithmetization-oriented permutations:

- **Anemoi over BN254 field**: Based on [anemoi-rust](https://github.com/anemoi-hash/anemoi-rust). Located in `capss/external/anemoi-rust`:
    ```bash
    cd capss/external/anemoi-rust
    cargo build --release
    ```
    
- **Anemoi over the 64-bit Goldilocks field**: Based on [hash_f64_benchmarks](https://github.com/anemoi-hash/hash_f64_benchmarks). Located in `capss/external/hash_f64_benchmarks`:
    ```bash
    cd capss/external/hash_f64_benchmarks
    cargo build --release
    ```

For a list of modifications applied to these libraries, see [capss/external/README.md](capss/external/README.md).

## ⚙️ Quick Usage

Use the following command to compile a benchmark script:
```bash
CAPSS_PERM={perm} CAPSS_FIELD={field} CAPSS_TRADEOFF={tradeoff} make bench
```

- `{perm}`: the permutation (for now, only `anemoi`),
- `{field}`: the base field (for now, `bn254` for the BN254's field, or `f64` for the 64-bit Goldilocks field),
- `{tradeoff}`: one of `short`, `default` and `fast`.

The command produces an executable `./bench` which benchmarks the signature scheme. For example:
```bash
./bench 100
```
generates and verifies 100 signatures.

## 📚 Documentation

A high-level documentation can be generated using [Doxygen](https://www.doxygen.nl/):
```bash
doxygen docs/Doxyfile
```
Output is placed in  `docs/html`.

📁 The documentation covers the following key components:
 * Field arithmetic: [smallwood/field-base.h](smallwood/field-base.h), [smallwood/field-vec.h](smallwood/field-vec.h), [smallwood/field-poly.h](smallwood/field-poly.h),
 * Merkle tree: [smallwood/merkle/generic/merkle.h](smallwood/merkle/generic/merkle.h),
 * Degree-enforcing commitment scheme (DECS): [smallwood/decs/decs.h](smallwood/decs/decs.h),
 * Polynomial commitment scheme (SmallWood-PCS): [smallwood/pcs/pcs.h](smallwood/pcs/pcs.h),
 * SmallWood proof system: [smallwood/smallwood/smallwood.h](smallwood/smallwood/smallwood.h),
 * CAPSS signature scheme: [capss/capss-sign.h](capss/capss-sign.h).

## 🔌 Integration Example

Here is a minimal example of using CAPSS to generate keys, sign, and verify signatures:

```c
#include <stdio.h>
#include <stdlib.h>
#include "capss-sign.h"

// Length of the message
#define MLEN 32

// The implementation requires a function "randombytes"
//   that will be used as a TRNG.
int randombytes(unsigned char* x, unsigned long long xlen) {
    for(unsigned long long j=0; j<xlen; j++)
        x[j] = (uint8_t) rand();
    return 0;
}

int main(void) {
    // Select the desired trade-off:
    // Short (0), Default (1), Fast (2)
    uint32_t capss_variant = 2;

    // Initialization
    uint8_t *pk = NULL;
    uint8_t *sk = NULL;
    uint8_t *sig = NULL;
    int ret = -1;

    // Get the key sizes and an upper bound of the signature size
    unsigned long long pk_bytesize, sk_bytesize, sig_max_bytesize;
    ret = capss_sign_get_sizes(capss_variant, &pk_bytesize, &sk_bytesize, &sig_max_bytesize);
    if(ret) goto err;

    // Allocate the keys and the signature
    pk = malloc(pk_bytesize);
    sk = malloc(sk_bytesize);
    sig = malloc(sig_max_bytesize);
    if(pk == NULL || sk == NULL || sig == NULL) goto err;

    // Generate the key pair
    ret = crypto_sign_keypair(pk, sk);
    if(ret) goto err;

    // Sign a message
    unsigned long long siglen;
    uint8_t m[MLEN] = {1, 2, 3, 4};
    ret = capss_sign_signature(capss_variant, sig, &siglen, m, MLEN, sk);
    if(ret) goto err;

    // Verify a message
    ret = capss_sign_verify(capss_variant, sig, siglen, m, MLEN, pk);
    if(ret) goto err;

    ret = 0;
err:
    free(pk);
    free(sk);
    free(sig);
    return ret;
}
```

## 📊 Benchmarks

Signature sizes and running times, compiled with Debian clang version 19.1.7 and benchmarked on an AMD Ryzen Threadripper PRO 7995WX.

| Perm.    | State Size | Field | Trade-off | Sign. Size | KeyGen    | Sign     | Verify |
|----------|------------|-------|-----------|------------|-----------|----------|--------|
| Anemoi-5 | 2, 4       | BN254 | Short     | 9092 B     | 0.1 ms    | 9900 ms  | 29 ms  |
| Anemoi-5 | 2, 4       | BN254 | Default   | 11486 B    | 0.1 ms    | 2460 ms  | 29 ms  |
| Anemoi-5 | 2, 4       | BN254 | Fast      | 12337 B    | 0.1 ms    | 710 ms   | 41 ms  |
| Anemoi-7 | 8          | F64   | Short     | 9084 B     | < 0.01 ms | 394 ms   | 1.2 ms |
| Anemoi-7 | 8          | F64   | Default   | 9715 B     | < 0.01 ms | 101 ms   | 1.4 ms |
| Anemoi-7 | 8          | F64   | Fast      | 10729 B    | < 0.01 ms | 29 ms    | 1.6 ms |

Signature sizes and running times, compiled with Apple clang version 16.0.0 and benchmarked on a MacBook Air with Apple M2 CPU.

| Perm.    | State Size | Field | Trade-off | Sign. Size | KeyGen    | Sign     | Verify |
|----------|------------|-------|-----------|------------|-----------|----------|--------|
| Anemoi-5 | 2, 4       | BN254 | Short     | 9092 B     | 0.3 ms    | 12300 ms | 36 ms  |
| Anemoi-5 | 2, 4       | BN254 | Default   | 11486 B    | 0.3 ms    | 2900 ms  | 37 ms  |
| Anemoi-5 | 2, 4       | BN254 | Fast      | 12337 B    | 0.3 ms    | 890 ms   | 51 ms  |
| Anemoi-7 | 8          | F64   | Short     | 9084 B     | < 0.05 ms | 580 ms   | 1.7 ms |
| Anemoi-7 | 8          | F64   | Default   | 9715 B     | < 0.05 ms | 160 ms   | 2.0 ms |
| Anemoi-7 | 8          | F64   | Fast      | 10729 B    | < 0.05 ms | 48 ms    | 2.4 ms |

## 📄 License

This project is licensed under the terms of Apache License (version 2.0).
