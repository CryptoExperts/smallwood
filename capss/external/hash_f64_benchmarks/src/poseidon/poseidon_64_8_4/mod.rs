// Copyright (c) 2021-2023 Toposware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cheetah::Fp;

/// Digest for Poseidon
mod digest;
/// Hasher for Poseidon
mod hasher;
/// MDS matrix for Poseidon
mod mds;
/// Round constants for Poseidon
mod round_constants;

pub use digest::PoseidonDigest;
pub use hasher::PoseidonHash;

use self::round_constants::ARK;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

// RESCUE CONSTANTS
// ================================================================================================

/// Function state is set to 8 field elements or 64 bytes;
/// 4 elements of the state are reserved for capacity
pub const STATE_WIDTH: usize = 8;
/// 8 elements of the state are reserved for rate
pub const RATE_WIDTH: usize = 4;

/// Seven elements (32-bytes) are returned as digest.
pub const DIGEST_SIZE: usize = 4;

/// The number of full rounds is set to 8 to provide 128-bit security level with security margin.
/// Computed using <https://github.com/Nashtare/goldilocks-hadeshash>. Note that considering
/// BBLP22 attack may reduce the actual security level.
pub const NUM_HALF_FULL_ROUNDS: usize = 4;

/// The number of partial rounds is set to 22 to provide 128-bit security level with security margin.
/// Computed using <https://github.com/Nashtare/goldilocks-hadeshash>. Note that considering
/// BBLP22 attack may reduce the actual security level.
pub const NUM_PARTIAL_ROUNDS: usize = 22;

// HELPER FUNCTIONS
// ================================================================================================

#[inline(always)]
/// Applies matrix-vector multiplication of the current
/// hash state with the Poseidon MDS matrix.
pub(crate) fn apply_mds(state: &mut [Fp; STATE_WIDTH]) {
    let mut result = [Fp::zero(); STATE_WIDTH];
    for (i, r) in result.iter_mut().enumerate() {
        for (j, s) in state.iter().enumerate() {
            *r += mds::MDS[i * STATE_WIDTH + j] * s;
        }
    }

    state.copy_from_slice(&result);
}

#[inline(always)]
/// Applies matrix-vector multiplication of the current
/// hash state with the Poseidon M_I matrix.
///
/// The matrix is in transposed form.
pub(crate) fn apply_mi(state: &mut [Fp; STATE_WIDTH]) {
    let mut result = [Fp::zero(); STATE_WIDTH];
    result[0] = state[0];
    for (i, r) in result.iter_mut().enumerate().skip(1) {
        for (j, s) in state.iter().enumerate().skip(1) {
            *r += mds::M_I[j * STATE_WIDTH + i] * s;
        }
    }

    state.copy_from_slice(&result);
}

#[inline(always)]
/// Applies matrix-vector multiplication of the current
/// hash state with the Poseidon MDS matrix.
pub(crate) fn cheap_matrix_mul(state: &mut [Fp; STATE_WIDTH], round: usize) {
    let res_0 = mds::W_HAT[NUM_PARTIAL_ROUNDS - round - 1]
        .iter()
        .zip(state.iter())
        .map(|(a, b)| a * b)
        .sum();
    let mul_row: [Fp; STATE_WIDTH] = mds::V_COL[NUM_PARTIAL_ROUNDS - round - 1]
        .iter()
        .map(|x| x * state[0])
        .collect::<Vec<Fp>>()
        .try_into()
        .unwrap();
    let mut add_row: [Fp; STATE_WIDTH] = mul_row
        .iter()
        .zip(state.iter())
        .map(|(x, y)| x + y)
        .collect::<Vec<Fp>>()
        .try_into()
        .unwrap();

    add_row[0] = res_0;
    *state = add_row;
}

#[inline(always)]
pub(crate) fn apply_full_sbox(state: &mut [Fp; STATE_WIDTH]) {
    state.iter_mut().for_each(pow_7);
}

#[inline(always)]
/// Applies exponentiation of the current element by 7
pub(crate) fn pow_7(x: &mut Fp) {
    let t2 = x.square();
    let t4 = t2.square();
    *x *= t2 * t4;
}

// RESCUE PERMUTATION
// ================================================================================================

/// Applies Poseidon permutation to the provided state.
pub(crate) fn apply_permutation(state: &mut [Fp; STATE_WIDTH]) {
    for i in 0..NUM_HALF_FULL_ROUNDS {
        apply_full_round(state, i);
    }

    apply_partial_rounds(state);

    for i in 0..NUM_HALF_FULL_ROUNDS {
        apply_full_round(state, NUM_PARTIAL_ROUNDS + NUM_HALF_FULL_ROUNDS + i);
    }
}

/// Poseidon full round function.
#[inline(always)]
pub(crate) fn apply_full_round(state: &mut [Fp; STATE_WIDTH], step: usize) {
    // determine which round constants to use
    let ark = round_constants::ARK[step % (2 * NUM_HALF_FULL_ROUNDS + NUM_PARTIAL_ROUNDS)];

    for i in 0..STATE_WIDTH {
        state[i] += ark[i];
    }

    apply_full_sbox(state);
    apply_mds(state);
}

/// Poseidon parial rounds function.
#[inline(always)]
pub(crate) fn apply_partial_rounds(state: &mut [Fp; STATE_WIDTH]) {
    // Initial constants addition
    let ark = ARK[NUM_HALF_FULL_ROUNDS];
    for i in 0..STATE_WIDTH {
        state[i] += ark[i];
    }

    apply_mi(state);

    for r in 0..NUM_PARTIAL_ROUNDS - 1 {
        pow_7(&mut state[0]);
        state[0] += ARK[NUM_HALF_FULL_ROUNDS + r + 1][0];
        cheap_matrix_mul(state, r);
    }

    // Last round
    pow_7(&mut state[0]);
    cheap_matrix_mul(state, NUM_PARTIAL_ROUNDS - 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;

    const INV_MDS: [Fp; STATE_WIDTH * STATE_WIDTH] = [
        Fp::new(2805993759005513109),
        Fp::new(9147504304377222261),
        Fp::new(10186790348949908466),
        Fp::new(14593887338999270741),
        Fp::new(754246057045137869),
        Fp::new(7032091965313564683),
        Fp::new(1582334361384110419),
        Fp::new(3397285519391949275),
        Fp::new(4854500844368524779),
        Fp::new(10098090430690574760),
        Fp::new(4776794689272624636),
        Fp::new(3824493624547477502),
        Fp::new(7880080516980618326),
        Fp::new(4784754318766922912),
        Fp::new(14682982126367272069),
        Fp::new(11048698798146136721),
        Fp::new(5728540872142733961),
        Fp::new(4480410893053871176),
        Fp::new(7179190529091425284),
        Fp::new(15335996858688509857),
        Fp::new(15979436587334165290),
        Fp::new(12698028971270457269),
        Fp::new(6656122556167367674),
        Fp::new(12716264955457801139),
        Fp::new(12533436531238616290),
        Fp::new(12459430509359113179),
        Fp::new(2872844384164053774),
        Fp::new(12834621551333490355),
        Fp::new(9489022079507955045),
        Fp::new(18093193875432168257),
        Fp::new(5762615595691466663),
        Fp::new(15045951268629825033),
        Fp::new(3696383206447714891),
        Fp::new(1656434615172430686),
        Fp::new(2582746653014872077),
        Fp::new(2466838098042195452),
        Fp::new(5554473912855733206),
        Fp::new(10581922113031912974),
        Fp::new(9702048306507161406),
        Fp::new(16027878415542328398),
        Fp::new(7695647565936145912),
        Fp::new(1841536849998909894),
        Fp::new(8207677599946233797),
        Fp::new(984489521811764857),
        Fp::new(10552204016652084011),
        Fp::new(3237030036341585333),
        Fp::new(12468842907344746891),
        Fp::new(3654783101089988815),
        Fp::new(16163777916280498426),
        Fp::new(11432309482951562857),
        Fp::new(3933091443675401050),
        Fp::new(10725456228255711758),
        Fp::new(936322145433461710),
        Fp::new(12414383897933578835),
        Fp::new(8468500546255459165),
        Fp::new(10294541324314731828),
        Fp::new(15262814459942401561),
        Fp::new(3044730265830350360),
        Fp::new(16180800363771856626),
        Fp::new(9094248992460577815),
        Fp::new(4758274616028203942),
        Fp::new(435133410534299348),
        Fp::new(9477478930753224752),
        Fp::new(6484449757558383252),
    ];

    /// Applies matrix-vector multiplication of the current
    /// hash state with the inverse Poseidon MDS matrix.
    fn apply_inv_mds(state: &mut [Fp; STATE_WIDTH]) {
        let mut result = [Fp::zero(); STATE_WIDTH];
        for (i, r) in result.iter_mut().enumerate() {
            for (j, s) in state.iter().enumerate() {
                *r += INV_MDS[i * STATE_WIDTH + j] * s;
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

            let state_copy = state;
            apply_mds(&mut state);

            // Check that matrix multiplication was consistent
            apply_inv_mds(&mut state);
            assert_eq!(state, state_copy);
        }
    }
}
