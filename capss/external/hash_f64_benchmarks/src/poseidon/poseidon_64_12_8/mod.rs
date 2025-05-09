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

/// Function state is set to 12 field elements or 96 bytes;
/// 4 elements of the state are reserved for capacity
pub const STATE_WIDTH: usize = 12;
/// 8 elements of the state are reserved for rate
pub const RATE_WIDTH: usize = 8;

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

/// Poseidon partial rounds function.
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
        Fp::new(7792246565304564677),
        Fp::new(6737692955818098982),
        Fp::new(15791360252969376700),
        Fp::new(18439545998547903789),
        Fp::new(16734825385473372180),
        Fp::new(10140624446356849193),
        Fp::new(11794865285518014666),
        Fp::new(2230841361506835732),
        Fp::new(11372108658544198209),
        Fp::new(5463705419431327078),
        Fp::new(10462729998602043300),
        Fp::new(14169413428913819094),
        Fp::new(5997431276131273385),
        Fp::new(8254576423272337335),
        Fp::new(16033387969251855141),
        Fp::new(2292547826487955779),
        Fp::new(17199532214101807449),
        Fp::new(16586075256567082741),
        Fp::new(10241182398805712556),
        Fp::new(5204617479282400644),
        Fp::new(14688815648913761756),
        Fp::new(13985631826627325426),
        Fp::new(17868697450992940962),
        Fp::new(11036519923621162644),
        Fp::new(9896089852381279076),
        Fp::new(1387108502390162567),
        Fp::new(17688415623332251784),
        Fp::new(7634188774953325023),
        Fp::new(13606558082653296513),
        Fp::new(14777160509404671947),
        Fp::new(6788199684845063803),
        Fp::new(1348730107459170050),
        Fp::new(1066492626911240989),
        Fp::new(5696798693567460950),
        Fp::new(1562940039928300678),
        Fp::new(2090872843902416284),
        Fp::new(17737991805099079339),
        Fp::new(12186236585264434022),
        Fp::new(13968489439304170277),
        Fp::new(432498034709121647),
        Fp::new(9331780971848569402),
        Fp::new(11555491758042323907),
        Fp::new(6998375352704439562),
        Fp::new(15691017284516246073),
        Fp::new(45908999545783440),
        Fp::new(15125408267219361928),
        Fp::new(261289812131805342),
        Fp::new(9586171282229604722),
        Fp::new(2397311691811073280),
        Fp::new(10197954120129172653),
        Fp::new(14600183771611113596),
        Fp::new(3605482521563051634),
        Fp::new(16029302701580524750),
        Fp::new(1226135555657393052),
        Fp::new(6909324053448291518),
        Fp::new(1827023493365315416),
        Fp::new(234701949265039456),
        Fp::new(8388881669051280590),
        Fp::new(7030311145045794101),
        Fp::new(7747498681316660123),
        Fp::new(7821684420983955256),
        Fp::new(8493650841321242208),
        Fp::new(7647971623325870677),
        Fp::new(3154293055571901568),
        Fp::new(14017475089668292079),
        Fp::new(4122528118531642118),
        Fp::new(15560327996390424880),
        Fp::new(4792356904158393082),
        Fp::new(9281414722545261503),
        Fp::new(10937210882112531833),
        Fp::new(6818621868472216190),
        Fp::new(15719822943961476000),
        Fp::new(3834722754751961386),
        Fp::new(10385241494937579384),
        Fp::new(17478627140018214727),
        Fp::new(13429271756391359880),
        Fp::new(14065075236679933468),
        Fp::new(18266867884476030582),
        Fp::new(4775486301608722194),
        Fp::new(876476359188388686),
        Fp::new(14767128742958403810),
        Fp::new(15844882719577225509),
        Fp::new(8519283286915585498),
        Fp::new(11698498222150031793),
        Fp::new(15188053448095911169),
        Fp::new(6645705634070887911),
        Fp::new(6242395348821973129),
        Fp::new(16700551771956619475),
        Fp::new(12796932097877915021),
        Fp::new(7654462724358120838),
        Fp::new(559667777046053579),
        Fp::new(15456746315386654702),
        Fp::new(1651119649478179189),
        Fp::new(1413062540372202446),
        Fp::new(4196568889468925049),
        Fp::new(3536209582714393691),
        Fp::new(12453522714558272688),
        Fp::new(17164760885518196768),
        Fp::new(14754139933057832558),
        Fp::new(5886297169670678940),
        Fp::new(16521770766367138568),
        Fp::new(5174792563293263237),
        Fp::new(5477730430629496026),
        Fp::new(7790640047902257329),
        Fp::new(15814239410257799513),
        Fp::new(514804492153574900),
        Fp::new(6301721944829213326),
        Fp::new(12203631079591342500),
        Fp::new(1669217711887079885),
        Fp::new(996819775750575929),
        Fp::new(16703385420178994115),
        Fp::new(2519703047823005265),
        Fp::new(5238692798856783646),
        Fp::new(8040057650649257265),
        Fp::new(4720421225720350085),
        Fp::new(18053938273462455849),
        Fp::new(18446002024572101283),
        Fp::new(10758243835291584425),
        Fp::new(1102905339657766003),
        Fp::new(10279509429056878793),
        Fp::new(16371278573463897314),
        Fp::new(14383702753852659506),
        Fp::new(7264817179969058640),
        Fp::new(6291629242201315744),
        Fp::new(3413072822723892630),
        Fp::new(8330448129229913210),
        Fp::new(16733566460094555816),
        Fp::new(16648503320933570732),
        Fp::new(12913647068447016510),
        Fp::new(5987553194213924258),
        Fp::new(4169046759276734476),
        Fp::new(15193813316217513582),
        Fp::new(8021949740434195035),
        Fp::new(14987886210393124529),
        Fp::new(7584512128970496137),
        Fp::new(10131114022341669237),
        Fp::new(12428957286055301081),
        Fp::new(2763957687298224050),
        Fp::new(3065178644418048920),
        Fp::new(3387175262199289738),
        Fp::new(793919983351123665),
        Fp::new(9497983756263453399),
        Fp::new(18260843074104487583),
        Fp::new(2733473456531985252),
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
