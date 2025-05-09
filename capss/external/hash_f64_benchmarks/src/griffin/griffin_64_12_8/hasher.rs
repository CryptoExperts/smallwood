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
        state[..DIGEST_SIZE].copy_from_slice(values[0].as_elements());
        state[DIGEST_SIZE..RATE_WIDTH].copy_from_slice(values[1].as_elements());
        apply_permutation(&mut state);

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
                Fp::new(17771657313902890713),
                Fp::new(3360565152887505109),
                Fp::new(14920795764112530715),
                Fp::new(11948124082919303809),
                Fp::new(16396603761839746764),
                Fp::new(3243269620326657209),
                Fp::new(104363946394052813),
                Fp::new(819693752694120646),
            ],
            [
                Fp::new(4157084691515810022),
                Fp::new(10092995359942746357),
                Fp::new(14051433692979846550),
                Fp::new(15252697767436088914),
                Fp::new(8406278126203652421),
                Fp::new(9974243852812003389),
                Fp::new(14431849249759515807),
                Fp::new(17738695084087342598),
            ],
            [
                Fp::new(8256777968747082795),
                Fp::new(3032940933448611586),
                Fp::new(12047065693351339349),
                Fp::new(13123828014888485388),
                Fp::new(46428796127139024),
                Fp::new(14494907641613551535),
                Fp::new(15491176602384310432),
                Fp::new(11996868801986686025),
            ],
            [
                Fp::new(8062268094781344300),
                Fp::new(1688892304944896586),
                Fp::new(14834177577132270151),
                Fp::new(1411148567742438240),
                Fp::new(6855435835121192246),
                Fp::new(8221761549712232529),
                Fp::new(15446584587114228155),
                Fp::new(11478391709864458327),
            ],
            [
                Fp::new(17306225126143684947),
                Fp::new(5567680237154001691),
                Fp::new(1793028295094355547),
                Fp::new(4683176632032883559),
                Fp::new(6425035995267110626),
                Fp::new(2129628804653128995),
                Fp::new(12688089499558318884),
                Fp::new(8883964305035016744),
            ],
            [
                Fp::new(114768060416439843),
                Fp::new(16409799019727298885),
                Fp::new(1645420222812002437),
                Fp::new(14633487532392768433),
                Fp::new(15323004431274953488),
                Fp::new(1338503479914752085),
                Fp::new(5700276276590021411),
                Fp::new(8364022411974124229),
            ],
            [
                Fp::new(10979465483354462441),
                Fp::new(2058290824681091682),
                Fp::new(17493112595358680440),
                Fp::new(8077726255699061547),
                Fp::new(15572712299332484832),
                Fp::new(18211521768577540369),
                Fp::new(16165478252397775916),
                Fp::new(5515305826255147431),
            ],
            [
                Fp::new(1672737816467183136),
                Fp::new(17480206366843666508),
                Fp::new(8853012710231053713),
                Fp::new(12312290772164368680),
                Fp::new(13437967714474921686),
                Fp::new(13717850891706659658),
                Fp::new(15270326914392657488),
                Fp::new(9539637570845629099),
            ],
            [
                Fp::new(6583206142012844060),
                Fp::new(7232575632032930729),
                Fp::new(3215121695798222387),
                Fp::new(14696728905206966804),
                Fp::new(15272567990705958540),
                Fp::new(11861335102543943956),
                Fp::new(10903431477316345101),
                Fp::new(4338173478137222251),
            ],
            [
                Fp::new(9659126348141737946),
                Fp::new(17574182766699483950),
                Fp::new(13171034791683016855),
                Fp::new(4619826977171985032),
                Fp::new(18182906134429339632),
                Fp::new(14036129254507324244),
                Fp::new(635063178769784611),
                Fp::new(12648414269975068444),
            ],
        ];

        // Generated from https://github.com/Nashtare/griffin-hash
        let output_data = [
            [
                Fp::new(13452764624003418152),
                Fp::new(12004385611710940293),
                Fp::new(2466541256145861705),
                Fp::new(9251577821251453908),
            ],
            [
                Fp::new(6896971564250207951),
                Fp::new(5979137225181012983),
                Fp::new(8867707565103778399),
                Fp::new(6156744349690566662),
            ],
            [
                Fp::new(5990042826014791586),
                Fp::new(6471028412767308619),
                Fp::new(4752615469696065629),
                Fp::new(14911139450496881778),
            ],
            [
                Fp::new(9095178637866566896),
                Fp::new(879078617142497838),
                Fp::new(8630804652779495725),
                Fp::new(57744654091316211),
            ],
            [
                Fp::new(8570166317963814401),
                Fp::new(244966149847689663),
                Fp::new(8284628172013182220),
                Fp::new(10786783503747514890),
            ],
            [
                Fp::new(16411757155253753541),
                Fp::new(1784049035925085043),
                Fp::new(3707371152985145120),
                Fp::new(16106178550535115862),
            ],
            [
                Fp::new(15891303737567774804),
                Fp::new(8819277979345881462),
                Fp::new(3282933022490543085),
                Fp::new(14339293756980561610),
            ],
            [
                Fp::new(5784586230418833207),
                Fp::new(14402119145397939608),
                Fp::new(14195215686061535539),
                Fp::new(135900229977946854),
            ],
            [
                Fp::new(15984720686373904106),
                Fp::new(5527823250854955748),
                Fp::new(8670665750936785522),
                Fp::new(307028059139367525),
            ],
            [
                Fp::new(6770492027509196775),
                Fp::new(7963385303692959306),
                Fp::new(14159592694668872802),
                Fp::new(5491566023573497838),
            ],
            [
                Fp::new(12003361447941798925),
                Fp::new(9530278486474645772),
                Fp::new(3806715025405010433),
                Fp::new(9803654258584046373),
            ],
            [
                Fp::new(14314789258725090097),
                Fp::new(626377718562134829),
                Fp::new(11808501522216899605),
                Fp::new(9373053118428973353),
            ],
        ];

        for (input, expected) in input_data.iter().zip(output_data) {
            assert_eq!(expected, GriffinHash::hash(input).to_elements());
        }
    }
}
