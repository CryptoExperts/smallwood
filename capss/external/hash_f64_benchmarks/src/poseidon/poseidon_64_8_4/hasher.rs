// Copyright (c) 2021-2023 Toposware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Hasher trait implementation for Poseidon

use core::convert::TryInto;

use super::digest::PoseidonDigest;
use super::{apply_permutation, DIGEST_SIZE, RATE_WIDTH, STATE_WIDTH};
use crate::traits::Hasher;

use cheetah::Fp;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// A Poseidon Hash over Fp
pub struct PoseidonHash {
    state: [Fp; STATE_WIDTH],
    idx: usize,
}

impl Default for PoseidonHash {
    fn default() -> Self {
        Self {
            state: [Fp::zero(); STATE_WIDTH],
            idx: 0,
        }
    }
}

impl Hasher<Fp> for PoseidonHash {
    type Digest = PoseidonDigest;

    fn hash(bytes: &[Fp]) -> Self::Digest {
        // initialize state to all zeros
        let mut state = [Fp::zero(); STATE_WIDTH];

        let mut i = 0;
        for &element in bytes.iter() {
            state[i] += element;
            i += 1;
            if i % RATE_WIDTH == 0 {
                apply_permutation(&mut state);
                i = 0;
            }
        }

        if i > 0 {
            state[i] += Fp::one();
            i += 1;

            while i % RATE_WIDTH != 0 {
                state[i] = Fp::zero();
                i += 1;
            }

            apply_permutation(&mut state);
        }

        PoseidonDigest::new(state[..DIGEST_SIZE].try_into().unwrap())
    }

    fn merge(values: &[Self::Digest; 2]) -> Self::Digest {
        let mut state = [Fp::zero(); STATE_WIDTH];
        let digest1 = values[0].as_elements();
        let digest2 = values[1].as_elements();
        // Uses Jive compression to fill the whole state and perform a single permutation
        state[..RATE_WIDTH].copy_from_slice(digest1);
        state[RATE_WIDTH..STATE_WIDTH].copy_from_slice(digest2);
        apply_permutation(&mut state);

        let mut result = [Fp::zero(); DIGEST_SIZE];
        for (i, r) in result.iter_mut().enumerate() {
            *r = digest1[i] + digest2[i] + state[i] + state[i + STATE_WIDTH / 2];
        }

        PoseidonDigest::new(state[..DIGEST_SIZE].try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poseidon_hash() {
        // Hardcoded input / output list generated from the
        // Sagemath code at <https://github.com/Nashtare/goldilocks-hadeshash>

        let input_data = [
            [Fp::zero(); 4],
            [Fp::one(); 4],
            [
                Fp::new(6569531088581465466),
                Fp::new(16633559500314554860),
                Fp::new(5886252279469044386),
                Fp::new(7254493620959209513),
            ],
            [
                Fp::new(14451671431242580637),
                Fp::new(12408955884259542783),
                Fp::new(6699700836437690609),
                Fp::new(6930173452022628101),
            ],
            [
                Fp::new(18395072982230283623),
                Fp::new(17422138460915468648),
                Fp::new(302928547891428217),
                Fp::new(17610521183302992333),
            ],
            [
                Fp::new(18068273339104138164),
                Fp::new(1911447746536554313),
                Fp::new(15648741352653160565),
                Fp::new(6668708735458596065),
            ],
            [
                Fp::new(11143425136806953711),
                Fp::new(18102105924703184544),
                Fp::new(7182071222255043774),
                Fp::new(9055541327943176217),
            ],
            [
                Fp::new(8655442662468800475),
                Fp::new(16505130820128335063),
                Fp::new(6493352664926984222),
                Fp::new(14967895957577870330),
            ],
            [
                Fp::new(9323549209372186683),
                Fp::new(10793117907827765253),
                Fp::new(182614859344069638),
                Fp::new(5967861484968599356),
            ],
            [
                Fp::new(10467738873745304557),
                Fp::new(16593721817482730487),
                Fp::new(16833487465053662813),
                Fp::new(13645546544866037446),
            ],
            [
                Fp::new(5197638704396892579),
                Fp::new(2260999998602521401),
                Fp::new(14339582556914007502),
                Fp::new(2059336184840445928),
            ],
            [
                Fp::new(11474010417503429257),
                Fp::new(6562199940984629687),
                Fp::new(1595612647018183356),
                Fp::new(15125081316530224427),
            ],
        ];

        // Generated from <https://github.com/Nashtare/goldilocks-hadeshash>
        let output_data = [
            [
                Fp::new(6072065921598985802),
                Fp::new(6854425184412602727),
                Fp::new(16486462955335846460),
                Fp::new(14227461248498733478),
            ],
            [
                Fp::new(14619925629145128426),
                Fp::new(7951183946068642775),
                Fp::new(4750566572738259090),
                Fp::new(13736235944952726619),
            ],
            [
                Fp::new(8881709354340870816),
                Fp::new(11022429230810803628),
                Fp::new(9919133748837268878),
                Fp::new(1940026939895304108),
            ],
            [
                Fp::new(11656023248066833884),
                Fp::new(13599899590527266009),
                Fp::new(17262648857581200042),
                Fp::new(3202400288309947193),
            ],
            [
                Fp::new(8458937849047354008),
                Fp::new(15444873881612443334),
                Fp::new(130710939793488335),
                Fp::new(11838447801522375581),
            ],
            [
                Fp::new(8917966575094799221),
                Fp::new(4277651356808142310),
                Fp::new(9750770388029453083),
                Fp::new(2476312540253634595),
            ],
            [
                Fp::new(14114517992873799837),
                Fp::new(3166448411688630311),
                Fp::new(3041711687728547973),
                Fp::new(305695350380950482),
            ],
            [
                Fp::new(5197363174065439425),
                Fp::new(5071853573198090823),
                Fp::new(13385993177584576567),
                Fp::new(13278633909187784046),
            ],
            [
                Fp::new(2676944379990996155),
                Fp::new(15747043270253121063),
                Fp::new(11125555704607582757),
                Fp::new(4016808809855679413),
            ],
            [
                Fp::new(15538529293906747595),
                Fp::new(13340638603716551260),
                Fp::new(12184144287914953677),
                Fp::new(9961483662172922422),
            ],
            [
                Fp::new(11165192549364424911),
                Fp::new(17848210351929386286),
                Fp::new(1085586923750759760),
                Fp::new(9547450804184665414),
            ],
            [
                Fp::new(10788604479956618845),
                Fp::new(7804054831326560044),
                Fp::new(13394728981872308276),
                Fp::new(3301074848628133393),
            ],
        ];

        for (input, expected) in input_data.iter().zip(output_data) {
            assert_eq!(expected, PoseidonHash::hash(input).to_elements());
        }
    }
}
