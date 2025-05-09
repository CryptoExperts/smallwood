// Copyright (c) 2021-2023 Toposware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate hash;
use cheetah::Fp;
use hash::traits::Hasher;
use hash::{poseidon_64_12_8, poseidon_64_8_4};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("poseidon-64-8-4 - merge", |bench| {
        let v: [poseidon_64_8_4::PoseidonDigest; 2] = [
            poseidon_64_8_4::PoseidonHash::hash(&[Fp::zero()]),
            poseidon_64_8_4::PoseidonHash::hash(&[Fp::one()]),
        ];

        bench.iter(|| poseidon_64_8_4::PoseidonHash::merge(black_box(&v)))
    });

    c.bench_function("poseidon-64-12-8 - merge", |bench| {
        let v: [poseidon_64_12_8::PoseidonDigest; 2] = [
            poseidon_64_12_8::PoseidonHash::hash(&[Fp::zero()]),
            poseidon_64_12_8::PoseidonHash::hash(&[Fp::one()]),
        ];

        bench.iter(|| poseidon_64_12_8::PoseidonHash::merge(black_box(&v)))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark);
criterion_main!(benches);
