// Copyright (c) 2021-2022 Toposware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Hasher trait implementation for Griffin

use core::convert::TryInto;

use super::digest::GriffinDigest;
use super::{apply_permutation, DIGEST_SIZE, RATE_WIDTH, STATE_WIDTH};
use crate::traits::Hasher;

use cheetah::Fp;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// A Griffin Hash over Fp
pub struct GriffinHash {
    state: [Fp; STATE_WIDTH],
    idx: usize,
}

impl Default for GriffinHash {
    fn default() -> Self {
        Self {
            state: [Fp::zero(); STATE_WIDTH],
            idx: 0,
        }
    }
}

impl Hasher<Fp> for GriffinHash {
    type Digest = GriffinDigest;

    fn hash(bytes: &[Fp]) -> Self::Digest {
        // initialize state to all zeros, except for the first element of the capacity part, which
        // is set to 1 conditionally on the input length. this is done so that adding zero elements
        // at the end of the list always results in a different hash.
        let mut state = [Fp::zero(); STATE_WIDTH];
        if bytes.len() % RATE_WIDTH != 0 {
            state[RATE_WIDTH] = Fp::one();
        }

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
            apply_permutation(&mut state);
        }

        GriffinDigest::new(state[..DIGEST_SIZE].try_into().unwrap())
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

        GriffinDigest::new(state[..DIGEST_SIZE].try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_griffin_hash() {
        // Hardcoded input / output list generated from the
        // Sagemath code at https://github.com/Nashtare/griffin-hash

        let input_data = [
            [Fp::zero(); 8],
            [Fp::one(); 8],
            [
                Fp::new(9343321687199583158),
                Fp::new(7913059135096528134),
                Fp::new(1738528429017635591),
                Fp::new(15202436977965565529),
                Fp::new(9096088132261665483),
                Fp::new(93971737309046170),
                Fp::new(16488170487704833731),
                Fp::new(9000758588749133025),
            ],
            [
                Fp::new(831286292267235615),
                Fp::new(18252505046174160544),
                Fp::new(12943398362525428376),
                Fp::new(7379696783801408461),
                Fp::new(9097972017494290664),
                Fp::new(7005326918161767110),
                Fp::new(16336074631882950335),
                Fp::new(1779116038745256765),
            ],
            [
                Fp::new(94604509777963642),
                Fp::new(3144301004971155635),
                Fp::new(10123350449087480803),
                Fp::new(9906813378360912264),
                Fp::new(3475913685037966463),
                Fp::new(3122362419548990007),
                Fp::new(2160066387844717340),
                Fp::new(14874299700965056891),
            ],
            [
                Fp::new(11538377067675855921),
                Fp::new(4607966423120164744),
                Fp::new(5369373683288745770),
                Fp::new(6981491060150355357),
                Fp::new(3585497387366854043),
                Fp::new(14112526192650888192),
                Fp::new(3493837240298181959),
                Fp::new(2838800122200635827),
            ],
            [
                Fp::new(3234342062337311871),
                Fp::new(14655473608225149666),
                Fp::new(3637068749501558055),
                Fp::new(16588940677546371492),
                Fp::new(6348529330443275906),
                Fp::new(13979816731655604574),
                Fp::new(9543102358679347366),
                Fp::new(9447279926249791615),
            ],
            [
                Fp::new(754407104284209767),
                Fp::new(14995999664519271305),
                Fp::new(16036238508800046144),
                Fp::new(1334607818368399560),
                Fp::new(6365854968820307490),
                Fp::new(6348461739171537157),
                Fp::new(9577772863721532153),
                Fp::new(4165530239749503205),
            ],
            [
                Fp::new(5360720265815806198),
                Fp::new(8218374057292591869),
                Fp::new(10265709187656861818),
                Fp::new(18009806364023019485),
                Fp::new(410423860111082310),
                Fp::new(17690372570339517544),
                Fp::new(9013889927823506361),
                Fp::new(18114855268301758535),
            ],
            [
                Fp::new(11857625426408891052),
                Fp::new(12543764539544385341),
                Fp::new(16772373164667604111),
                Fp::new(1300029421926931452),
                Fp::new(7826061005126113898),
                Fp::new(7052201010108553852),
                Fp::new(1603182656911290374),
                Fp::new(3941429400317832633),
            ],
            [
                Fp::new(5483691145966367361),
                Fp::new(4461516906707966901),
                Fp::new(12926205722061796874),
                Fp::new(7574384293848398080),
                Fp::new(12539376167004392328),
                Fp::new(12967569845511369877),
                Fp::new(1359513970218543879),
                Fp::new(4027739666316204293),
            ],
            [
                Fp::new(16724792196646807249),
                Fp::new(6843775795069184305),
                Fp::new(8641746343131394074),
                Fp::new(9036460881341054122),
                Fp::new(6043669070806422990),
                Fp::new(4203084679129895786),
                Fp::new(17098237140453352636),
                Fp::new(15049247262672350175),
            ],
        ];

        // Generated from https://github.com/Nashtare/griffin-hash
        let output_data = [
            [
                Fp::new(9551397893192414667),
                Fp::new(15266765102125302135),
                Fp::new(9818251175799610338),
                Fp::new(18045524617384237498),
            ],
            [
                Fp::new(13419752039104747543),
                Fp::new(12481257269128394467),
                Fp::new(13609860115618360011),
                Fp::new(7372657541286901247),
            ],
            [
                Fp::new(1671894167325030026),
                Fp::new(15577530542061619870),
                Fp::new(8919840952265799179),
                Fp::new(13706152502454287023),
            ],
            [
                Fp::new(8884327016686669867),
                Fp::new(7244285883568807937),
                Fp::new(5410539287553896389),
                Fp::new(8783702785298817900),
            ],
            [
                Fp::new(5676321884849163111),
                Fp::new(4478576866797147380),
                Fp::new(3009153632452774330),
                Fp::new(11629595963193815340),
            ],
            [
                Fp::new(14742817487612001564),
                Fp::new(9987207354986308),
                Fp::new(15578683490205370764),
                Fp::new(14096883893073498603),
            ],
            [
                Fp::new(8984984547635245463),
                Fp::new(10682336535863109541),
                Fp::new(7945485140109149475),
                Fp::new(15792367082075391925),
            ],
            [
                Fp::new(12772850327782817890),
                Fp::new(10860240774195884618),
                Fp::new(16979953104775149839),
                Fp::new(5959548902217872229),
            ],
            [
                Fp::new(8686382912375820471),
                Fp::new(18186292718889876927),
                Fp::new(5388848570641927240),
                Fp::new(9959523034007085259),
            ],
            [
                Fp::new(14781929927510875308),
                Fp::new(4397834034823872522),
                Fp::new(5669184268113948163),
                Fp::new(12245316814171151223),
            ],
            [
                Fp::new(8837219267974566664),
                Fp::new(10087669433745268057),
                Fp::new(12166070078658017261),
                Fp::new(16650566031963927162),
            ],
            [
                Fp::new(14414264289327901348),
                Fp::new(5895976031920885585),
                Fp::new(17738808838832685993),
                Fp::new(10792939709821248274),
            ],
        ];

        for (input, expected) in input_data.iter().zip(output_data) {
            assert_eq!(expected, GriffinHash::hash(input).to_elements());
        }
    }
}
