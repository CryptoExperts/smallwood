// Copyright (c) 2021-2022 Toposware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cheetah::fp_arith_utils::reduce_u96;
use cheetah::Fp;

/// Digest for Griffin
mod digest;
/// Hasher for Griffin
mod hasher;
/// MDS matrix for Griffin
mod mds;
/// Non-linear layer for Griffin
mod non_linear;
/// Round constants for Griffin
mod round_constants;

pub use digest::GriffinDigest;
pub use hasher::GriffinHash;

// ANEMOI CONSTANTS
// ================================================================================================

/// Function state is set to 12 field elements or 96 bytes.
/// 4 elements of the state are reserved for capacity.
pub const STATE_WIDTH: usize = 12;
/// 8 elements of the state are reserved for rate.
pub const RATE_WIDTH: usize = 8;
/// 8 elements of the state are reserved for rate.
pub const CAPACITY_WIDTH: usize = STATE_WIDTH - RATE_WIDTH;

/// Four elements (32-bytes) are returned as digest.
pub const DIGEST_SIZE: usize = 4;

/// The number of rounds is set to 8 to provide 128-bit security level
/// with 20% security margin.
pub const NUM_HASH_ROUNDS: usize = 8;

// HELPER FUNCTIONS
// ================================================================================================

#[inline(always)]
/// Applies the Griffin non-linear layer
/// to the current hash state.
pub(crate) fn apply_non_linear(state: &mut [Fp; STATE_WIDTH]) {
    non_linear::pow_inv_d(&mut state[0]);
    non_linear::pow_d(&mut state[1]);

    let l2 = linear_function(2, state[0], state[1], Fp::zero());
    state[2] *= l2.square() + non_linear::ALPHA[0] * l2 + non_linear::BETA[0];

    let l3 = linear_function(3, state[0], state[1], state[2]);
    state[3] *= l3.square() + non_linear::ALPHA[1] * l3 + non_linear::BETA[1];

    let l4 = linear_function(4, state[0], state[1], state[3]);
    state[4] *= l4.square() + non_linear::ALPHA[2] * l4 + non_linear::BETA[2];

    let l5 = linear_function(5, state[0], state[1], state[4]);
    state[5] *= l5.square() + non_linear::ALPHA[3] * l5 + non_linear::BETA[3];

    let l6 = linear_function(6, state[0], state[1], state[5]);
    state[6] *= l6.square() + non_linear::ALPHA[4] * l6 + non_linear::BETA[4];

    let l7 = linear_function(7, state[0], state[1], state[6]);
    state[7] *= l7.square() + non_linear::ALPHA[5] * l7 + non_linear::BETA[5];

    let l8 = linear_function(8, state[0], state[1], state[7]);
    state[8] *= l8.square() + non_linear::ALPHA[6] * l8 + non_linear::BETA[6];

    let l9 = linear_function(9, state[0], state[1], state[8]);
    state[9] *= l9.square() + non_linear::ALPHA[7] * l9 + non_linear::BETA[7];

    let l10 = linear_function(10, state[0], state[1], state[9]);
    state[10] *= l10.square() + non_linear::ALPHA[8] * l10 + non_linear::BETA[8];

    let l11 = linear_function(11, state[0], state[1], state[10]);
    state[11] *= l11.square() + non_linear::ALPHA[9] * l11 + non_linear::BETA[9];
}

#[inline(always)]
fn linear_function(index: u64, z0: Fp, z1: Fp, z2: Fp) -> Fp {
    Fp::from_raw_unchecked(index - 1) * z0 + z1 + z2
}

#[inline(always)]
/// Applies matrix-vector multiplication of the current
/// hash state with the Griffin MDS matrix.
pub(crate) fn apply_mds(state: &mut [Fp; STATE_WIDTH]) {
    let x: [u128; STATE_WIDTH] = [
        state[0].output_unreduced_internal() as u128,
        state[1].output_unreduced_internal() as u128,
        state[2].output_unreduced_internal() as u128,
        state[3].output_unreduced_internal() as u128,
        state[4].output_unreduced_internal() as u128,
        state[5].output_unreduced_internal() as u128,
        state[6].output_unreduced_internal() as u128,
        state[7].output_unreduced_internal() as u128,
        state[8].output_unreduced_internal() as u128,
        state[9].output_unreduced_internal() as u128,
        state[10].output_unreduced_internal() as u128,
        state[11].output_unreduced_internal() as u128,
    ];

    const T4: usize = STATE_WIDTH / 4;

    let mut result = [0u128; STATE_WIDTH];

    // Apply first matrix
    for i in 0..T4 {
        let start_index = i * 4;
        let mut t0 = x[start_index];
        t0 += x[start_index + 1];
        let mut t1 = x[start_index + 2];
        t1 += x[start_index + 3];
        let mut t2 = x[start_index + 1];
        t2 <<= 1;
        t2 += t1;
        let mut t3 = x[start_index + 3];
        t3 <<= 1;
        t3 += t0;
        let mut t4 = t1;
        t4 <<= 2;
        t4 += t3;
        let mut t5 = t0;
        t5 <<= 2;
        t5 += t2;
        let mut t6 = t3;
        t6 += t5;
        let mut t7 = t2;
        t7 += t4;

        result[start_index] = t6;
        result[start_index + 1] = t5;
        result[start_index + 2] = t7;
        result[start_index + 3] = t4;
    }

    // Apply second matrix
    let mut stored = [0u128; 4];
    for l in 0..4 {
        stored[l] = result[l];
        for j in 1..T4 {
            stored[l] += result[4 * j + l];
        }
    }

    // Final addition and modular reduction
    state.copy_from_slice(&[
        Fp::from_raw_unchecked(reduce_u96(result[0] + stored[0])),
        Fp::from_raw_unchecked(reduce_u96(result[1] + stored[1])),
        Fp::from_raw_unchecked(reduce_u96(result[2] + stored[2])),
        Fp::from_raw_unchecked(reduce_u96(result[3] + stored[3])),
        Fp::from_raw_unchecked(reduce_u96(result[4] + stored[0])),
        Fp::from_raw_unchecked(reduce_u96(result[5] + stored[1])),
        Fp::from_raw_unchecked(reduce_u96(result[6] + stored[2])),
        Fp::from_raw_unchecked(reduce_u96(result[7] + stored[3])),
        Fp::from_raw_unchecked(reduce_u96(result[8] + stored[0])),
        Fp::from_raw_unchecked(reduce_u96(result[9] + stored[1])),
        Fp::from_raw_unchecked(reduce_u96(result[10] + stored[2])),
        Fp::from_raw_unchecked(reduce_u96(result[11] + stored[3])),
    ]);
}

// ANEMOI PERMUTATION
// ================================================================================================

/// Applies Griffin permutation to the provided state.
pub(crate) fn apply_permutation(state: &mut [Fp; STATE_WIDTH]) {
    for i in 0..NUM_HASH_ROUNDS - 1 {
        apply_round(state, i);
    }
    apply_non_linear(state);
    apply_mds(state);
}

/// Griffin round function
#[inline(always)]
pub(crate) fn apply_round(state: &mut [Fp; STATE_WIDTH], step: usize) {
    // determine which round constants to use
    let c = &round_constants::ARK[step % (NUM_HASH_ROUNDS - 1)];

    apply_non_linear(state);
    apply_mds(state);

    for i in 0..STATE_WIDTH {
        state[i] += c[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;

    fn mat_vec_mult(state: &mut [Fp; STATE_WIDTH]) {
        let mut result = [Fp::zero(); STATE_WIDTH];
        for (i, r) in result.iter_mut().enumerate() {
            for (j, s) in state.iter().enumerate() {
                *r += Fp::new(mds::MDS[i * STATE_WIDTH + j]) * s;
            }
        }

        state.copy_from_slice(&result);
    }

    #[test]
    fn test_mds() {
        let mut state = [Fp::zero(); STATE_WIDTH];
        let mut rng = OsRng;

        for _ in 0..100 {
            for s in state.iter_mut() {
                *s = Fp::random(&mut rng);
            }

            let mut state_copy = state;
            apply_mds(&mut state);

            // Check that matrix multiplication was consistent
            mat_vec_mult(&mut state_copy);
            assert_eq!(state, state_copy);
        }
    }

    #[test]
    fn test_d() {
        let mut rng = OsRng;

        for _ in 0..100 {
            let mut x = Fp::random(&mut rng);
            let x_copy = x;

            non_linear::pow_d(&mut x);
            non_linear::pow_inv_d(&mut x);

            assert_eq!(x, x_copy);
        }
    }
}
