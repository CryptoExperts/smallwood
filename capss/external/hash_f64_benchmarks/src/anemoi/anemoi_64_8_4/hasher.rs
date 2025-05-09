// Copyright (c) 2021-2022 Toposware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Hasher trait implementation for Anemoi

use core::convert::TryInto;

use super::digest::AnemoiDigest;
use super::{apply_permutation, DIGEST_SIZE, NUM_COLUMNS, RATE_WIDTH, STATE_WIDTH};
use crate::traits::Hasher;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use cheetah::Fp;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// A Anemoi Hash over Fp
pub struct AnemoiHash {
    state: [Fp; STATE_WIDTH],
    idx: usize,
}

impl Default for AnemoiHash {
    fn default() -> Self {
        Self {
            state: [Fp::zero(); STATE_WIDTH],
            idx: 0,
        }
    }
}

impl Hasher<Fp> for AnemoiHash {
    type Digest = AnemoiDigest;

    fn hash(bytes: &[Fp]) -> Self::Digest {
        // initialize state to all zeros.
        let mut state = [Fp::zero(); STATE_WIDTH];

        let sigma = if bytes.len() % RATE_WIDTH == 0 {
            Fp::one()
        } else {
            Fp::zero()
        };

        let mut i = 0;
        for &element in bytes.iter() {
            state[i] += element;
            i += 1;
            if i % RATE_WIDTH == 0 {
                apply_permutation(&mut state);
                i = 0;
            }
        }

        // If the message length is not a multiple of RATE_WIDTH, we append 1 to the rate cell
        // next to the one where we previously appended the last message element. This is
        // guaranted to be in the rate registers (i.e. to not require an extra permutation before
        // adding this constant) if sigma is equal to zero.
        if sigma.is_zero().into() {
            state[i] += Fp::one();
            apply_permutation(&mut state);
        }

        // We then add sigma to the last capacity register of the capacity.
        state[STATE_WIDTH - 1] += sigma;

        AnemoiDigest::new(state[..DIGEST_SIZE].try_into().unwrap())
    }

    // This merge function uses the compression approach of Anemoi-Jive
    // to save one Anemoi permutation call , which would be necessary if
    // using the regular Anemoi-Sponge to absorb two digests, both of
    // size RATE_WIDTH.
    fn merge(values: &[Self::Digest; 2]) -> Self::Digest {
        let mut state = [Fp::zero(); STATE_WIDTH];
        let digest1 = values[0].as_elements();
        let digest2 = values[1].as_elements();
        state[..RATE_WIDTH].copy_from_slice(digest1);
        state[RATE_WIDTH..STATE_WIDTH].copy_from_slice(digest2);
        apply_permutation(&mut state);

        let mut result = [Fp::zero(); DIGEST_SIZE];
        for (i, r) in result.iter_mut().enumerate() {
            *r = digest1[i] + digest2[i] + state[i] + state[i + NUM_COLUMNS];
        }

        AnemoiDigest::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anemoi_hash() {
        // Hardcoded input / output list generated from the
        // Sagemath code at https://github.com/anemoi/anemoi-hash/

        let input_data = [
            vec![Fp::zero(); 8],
            vec![Fp::one(); 8],
            vec![
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
                Fp::one(),
                Fp::one(),
                Fp::one(),
                Fp::one(),
            ],
            vec![
                Fp::one(),
                Fp::one(),
                Fp::one(),
                Fp::one(),
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
            ],
            vec![Fp::new(17034304680378990593)],
            vec![Fp::new(14717522243836148645), Fp::new(13083744320516212099)],
            vec![
                Fp::new(12636374470493764585),
                Fp::new(4633685103624134705),
                Fp::new(3387113220078373436),
            ],
            vec![
                Fp::new(8456646190515084682),
                Fp::new(13422660296625058046),
                Fp::new(6412264356237719015),
                Fp::new(15681324668660747245),
            ],
            vec![
                Fp::new(5024098825481674854),
                Fp::new(17074804226935063103),
                Fp::new(12706716815026291874),
                Fp::new(15111404609115389377),
                Fp::new(14660803971611056841),
            ],
            vec![
                Fp::new(12593193490016384806),
                Fp::new(18163695353480745896),
                Fp::new(12379842404544411425),
                Fp::new(15341182951106044393),
                Fp::new(1499922380158571885),
                Fp::new(4387632416457430195),
            ],
        ];

        let output_data = [
            [
                Fp::new(2147571211920283164),
                Fp::new(3500197923981355332),
                Fp::new(12025754856179576335),
                Fp::new(5191900753661602732),
            ],
            [
                Fp::new(4799501315433889617),
                Fp::new(11825564577444848906),
                Fp::new(12699451361625258416),
                Fp::new(4403883436189857244),
            ],
            [
                Fp::new(14917676365189156135),
                Fp::new(9690281049492942316),
                Fp::new(15405492502585326016),
                Fp::new(18257629936304112975),
            ],
            [
                Fp::new(9101898144046340605),
                Fp::new(4963483388223873658),
                Fp::new(8085963357682719110),
                Fp::new(3413652506285881800),
            ],
            [
                Fp::new(10934120972896358413),
                Fp::new(3742264782653251191),
                Fp::new(4906328105106391092),
                Fp::new(1813825698241223332),
            ],
            [
                Fp::new(6789700677830222583),
                Fp::new(8009112044300927691),
                Fp::new(9757319550680499127),
                Fp::new(16564808397257287101),
            ],
            [
                Fp::new(16029117184986650692),
                Fp::new(11512605906904701149),
                Fp::new(5230546315434683976),
                Fp::new(10423628038384397897),
            ],
            [
                Fp::new(4845318800019578716),
                Fp::new(10770568660375899882),
                Fp::new(3321688296942833592),
                Fp::new(4907970827800013019),
            ],
            [
                Fp::new(5721243346329843792),
                Fp::new(3013112151732871576),
                Fp::new(15739298638007301605),
                Fp::new(3354525661329374476),
            ],
            [
                Fp::new(4643751874746640841),
                Fp::new(15147160842484799508),
                Fp::new(2070625496005120821),
                Fp::new(5404045163787980390),
            ],
        ];

        for (input, expected) in input_data.iter().zip(output_data) {
            assert_eq!(expected, AnemoiHash::hash(input).to_elements());
        }
    }
}
