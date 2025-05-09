//###################   The CAPSS Framework (2025)   ##################//
//   This file has been modified in the context of the implementation  //
// of the CAPSS framework. Please refer to available the change log    //
// for the details.                                                    //
//#####################################################################//

// Copyright (c) 2021-2022 Toposware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::f64_utils::apply_rescue_inv_sbox;
use cheetah::fp_arith_utils::reduce_u96;
use cheetah::Fp;

/// Digest for Anemoi
mod digest;
/// Hasher for Anemoi
mod hasher;
/// MDS matrix for Anemoi
mod mds;
/// Round constants for Anemoi
mod round_constants;
/// S-Box for Anemoi
mod sbox;
/// Wrapper for Anemoi
mod wrapper;

pub use digest::AnemoiDigest;
pub use hasher::AnemoiHash;

// ANEMOI CONSTANTS
// ================================================================================================

/// Function state is set to 8 field elements or 64 bytes.
/// 4 elements of the state are reserved for capacity.
pub const STATE_WIDTH: usize = 8;
/// 4 elements of the state are reserved for rate.
pub const RATE_WIDTH: usize = 4;

/// The state is divided into two even-length rows.
pub const NUM_COLUMNS: usize = 4;

/// Four elements (32-bytes) are returned as digest.
pub const DIGEST_SIZE: usize = 4;

/// The number of rounds is set to 11 to provide 128-bit security level.
pub const NUM_HASH_ROUNDS: usize = 11;

// HELPER FUNCTIONS
// ================================================================================================

#[inline(always)]
/// Applies application of the Anemoi S-Box
/// to the current hash state.
pub(crate) fn apply_sbox(state: &mut [Fp; STATE_WIDTH]) {
    let mut x: [Fp; NUM_COLUMNS] = state[..NUM_COLUMNS].try_into().unwrap();
    let mut y: [Fp; NUM_COLUMNS] = state[NUM_COLUMNS..].try_into().unwrap();

    x.iter_mut()
        .enumerate()
        .for_each(|(i, t)| *t -= y[i].square().mul_by_u32(sbox::BETA));

    let mut x_alpha_inv = x;
    apply_rescue_inv_sbox(&mut x_alpha_inv);

    y.iter_mut()
        .enumerate()
        .for_each(|(i, t)| *t -= x_alpha_inv[i]);

    x.iter_mut()
        .enumerate()
        .for_each(|(i, t)| *t += y[i].square().mul_by_u32(sbox::BETA) + sbox::DELTA);

    state[..NUM_COLUMNS].copy_from_slice(&x);
    state[NUM_COLUMNS..].copy_from_slice(&y);
}

#[inline(always)]
/// Applies matrix-vector multiplication of the current
/// hash state with the Anemoi MDS matrix.
pub(crate) fn apply_mds(state: &mut [Fp; STATE_WIDTH]) {
    let mut x: [u128; NUM_COLUMNS] = [
        state[0].output_unreduced_internal() as u128,
        state[1].output_unreduced_internal() as u128,
        state[2].output_unreduced_internal() as u128,
        state[3].output_unreduced_internal() as u128,
    ];
    // The second vector is first permuted
    let mut y: [u128; NUM_COLUMNS] = [
        state[5].output_unreduced_internal() as u128,
        state[6].output_unreduced_internal() as u128,
        state[7].output_unreduced_internal() as u128,
        state[4].output_unreduced_internal() as u128,
    ];

    // MDS layer over X vector

    x[0] += x[1];
    x[2] += x[3];
    x[3] += 7 * x[0];
    x[1] = 7 * (x[1] + x[2]);
    x[0] += x[1];
    x[2] += 7 * x[3];
    x[1] += x[2];
    x[3] += x[0];

    // MDS layer over Y vector

    y[0] += y[1];
    y[2] += y[3];
    y[3] += 7 * y[0];
    y[1] = 7 * (y[1] + y[2]);
    y[0] += y[1];
    y[2] += 7 * y[3];
    y[1] += y[2];
    y[3] += y[0];

    // Before applying modular reduction, we perform a final
    // Pseudo-Hadamard transform on each pair (x_i, y_i).

    y[0] += x[0];
    y[1] += x[1];
    y[2] += x[2];
    y[3] += x[3];

    x[0] += y[0];
    x[1] += y[1];
    x[2] += y[2];
    x[3] += y[3];

    state[0] = Fp::from_raw_unchecked(reduce_u96(x[0]));
    state[1] = Fp::from_raw_unchecked(reduce_u96(x[1]));
    state[2] = Fp::from_raw_unchecked(reduce_u96(x[2]));
    state[3] = Fp::from_raw_unchecked(reduce_u96(x[3]));

    state[4] = Fp::from_raw_unchecked(reduce_u96(y[0]));
    state[5] = Fp::from_raw_unchecked(reduce_u96(y[1]));
    state[6] = Fp::from_raw_unchecked(reduce_u96(y[2]));
    state[7] = Fp::from_raw_unchecked(reduce_u96(y[3]));
}

// ANEMOI PERMUTATION
// ================================================================================================

/// Applies Anemoi permutation to the provided state.
pub(crate) fn apply_permutation(state: &mut [Fp; STATE_WIDTH]) {
    for i in 0..NUM_HASH_ROUNDS {
        apply_round(state, i);
    }

    apply_mds(state)
}

/// Anemoi round function
#[inline(always)]
pub(crate) fn apply_round(state: &mut [Fp; STATE_WIDTH], step: usize) {
    // determine which round constants to use
    let c = &round_constants::C[step % NUM_HASH_ROUNDS];
    let d = &round_constants::D[step % NUM_HASH_ROUNDS];

    for i in 0..NUM_COLUMNS {
        state[i] += c[i];
        state[NUM_COLUMNS + i] += d[i];
    }

    apply_mds(state);
    apply_sbox(state);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn apply_naive_mds(state: &mut [Fp; STATE_WIDTH]) {
        let x: [Fp; NUM_COLUMNS] = state[..NUM_COLUMNS].try_into().unwrap();
        let mut y: [Fp; NUM_COLUMNS] = [Fp::zero(); NUM_COLUMNS];
        y[0..NUM_COLUMNS - 1].copy_from_slice(&state[NUM_COLUMNS + 1..]);
        y[NUM_COLUMNS - 1] = state[NUM_COLUMNS];

        let mut result = [Fp::zero(); STATE_WIDTH];
        for (i, r) in result.iter_mut().enumerate().take(NUM_COLUMNS) {
            for (j, s) in x.into_iter().enumerate().take(NUM_COLUMNS) {
                *r += s.mul_by_u32(mds::MDS[i * NUM_COLUMNS + j]);
            }
        }
        for (i, r) in result.iter_mut().enumerate().skip(NUM_COLUMNS) {
            for (j, s) in y.into_iter().enumerate() {
                *r += s.mul_by_u32(mds::MDS[(i - NUM_COLUMNS) * NUM_COLUMNS + j]);
            }
        }

        // Pseudo-Hadamard transform
        for i in 0..NUM_COLUMNS {
            result[i + NUM_COLUMNS] += result[i];
            result[i] += result[i + NUM_COLUMNS];
        }

        state.copy_from_slice(&result);
    }

    #[test]
    fn test_sbox() {
        // Hardcoded input / output list generated from the
        // Sagemath code at https://github.com/Nashtare/anemoi-hash/

        let mut input = [
            [Fp::zero(); 8],
            [Fp::one(); 8],
            [
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
                Fp::one(),
                Fp::one(),
                Fp::one(),
                Fp::one(),
            ],
            [
                Fp::one(),
                Fp::one(),
                Fp::one(),
                Fp::one(),
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
            ],
            [
                Fp::new(2000766362960169278),
                Fp::new(4845355602429368405),
                Fp::new(12562369912622388673),
                Fp::new(10659713775963144853),
                Fp::new(14027961696779974157),
                Fp::new(13410746108923109767),
                Fp::new(3961452001650000406),
                Fp::new(1639062069807858848),
            ],
            [
                Fp::new(14853235963268149062),
                Fp::new(17441936912918247583),
                Fp::new(7643291178810327459),
                Fp::new(8164634436074463438),
                Fp::new(10155801445575806130),
                Fp::new(6506129476148692149),
                Fp::new(16140738103380957345),
                Fp::new(5352974113421264521),
            ],
            [
                Fp::new(8114638517495889734),
                Fp::new(14179614979074726382),
                Fp::new(16082096889175193489),
                Fp::new(6324904139276506946),
                Fp::new(9991788656300841835),
                Fp::new(2807980177118468990),
                Fp::new(1336611829007040800),
                Fp::new(15940575012363858511),
            ],
            [
                Fp::new(13627868705508249911),
                Fp::new(9114632715714350564),
                Fp::new(13473486566516063716),
                Fp::new(6895988288022455316),
                Fp::new(16315504854304818404),
                Fp::new(5737984448949899610),
                Fp::new(4241664203384419597),
                Fp::new(12594880768523275260),
            ],
            [
                Fp::new(13068376737647104569),
                Fp::new(10692562188074272981),
                Fp::new(11299873151842561444),
                Fp::new(17672246578671489634),
                Fp::new(14875209631334205326),
                Fp::new(12027570670710720717),
                Fp::new(18022958450103541524),
                Fp::new(2420778927169955496),
            ],
            [
                Fp::new(4947920935843447728),
                Fp::new(12098785146176647902),
                Fp::new(17166166198664194317),
                Fp::new(11166939034433399948),
                Fp::new(2136319990591049768),
                Fp::new(2736867774456213062),
                Fp::new(13507021019950436304),
                Fp::new(13232604862253703587),
            ],
        ];

        let output = [
            [
                Fp::new(2635249152773512046),
                Fp::new(2635249152773512046),
                Fp::new(2635249152773512046),
                Fp::new(2635249152773512046),
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
            ],
            [
                Fp::new(17136669903572771321),
                Fp::new(17136669903572771321),
                Fp::new(17136669903572771321),
                Fp::new(17136669903572771321),
                Fp::new(9739452640566982996),
                Fp::new(9739452640566982996),
                Fp::new(9739452640566982996),
                Fp::new(9739452640566982996),
            ],
            [
                Fp::new(6928912281476859504),
                Fp::new(6928912281476859504),
                Fp::new(6928912281476859504),
                Fp::new(6928912281476859504),
                Fp::new(5829874566404923654),
                Fp::new(5829874566404923654),
                Fp::new(5829874566404923654),
                Fp::new(5829874566404923654),
            ],
            [
                Fp::new(2635249152773512054),
                Fp::new(2635249152773512054),
                Fp::new(2635249152773512054),
                Fp::new(2635249152773512054),
                Fp::new(18446744069414584320),
                Fp::new(18446744069414584320),
                Fp::new(18446744069414584320),
                Fp::new(18446744069414584320),
            ],
            [
                Fp::new(17595368791915918314),
                Fp::new(1750158132891253421),
                Fp::new(7934148582278531157),
                Fp::new(18355253938099149710),
                Fp::new(15847976893783333230),
                Fp::new(15140732409383769781),
                Fp::new(16107133749029117805),
                Fp::new(12614763451833978886),
            ],
            [
                Fp::new(5302779753731872284),
                Fp::new(3887178818269483953),
                Fp::new(85381178363327055),
                Fp::new(5227723577952764442),
                Fp::new(7294298551847701990),
                Fp::new(1944773062116157281),
                Fp::new(13095164951666319949),
                Fp::new(15417746668681697101),
            ],
            [
                Fp::new(4704698942725233197),
                Fp::new(11389576642300888261),
                Fp::new(13747732256400828369),
                Fp::new(2609414885706899257),
                Fp::new(12042853374652098754),
                Fp::new(10471497707017371526),
                Fp::new(42973467736111982),
                Fp::new(7195700627358816372),
            ],
            [
                Fp::new(12437071314292813312),
                Fp::new(5221381167752175556),
                Fp::new(8516204803399229117),
                Fp::new(6334603267745271456),
                Fp::new(3115181416705736527),
                Fp::new(17749852098054777624),
                Fp::new(4832067050365806680),
                Fp::new(12570901017104873609),
            ],
            [
                Fp::new(15753092989584517562),
                Fp::new(8311532108724994783),
                Fp::new(15966800123754853117),
                Fp::new(11207348653829308640),
                Fp::new(8616892148280334855),
                Fp::new(624105956256411711),
                Fp::new(15613527877396674140),
                Fp::new(2080829150971069173),
            ],
            [
                Fp::new(7154418693863441898),
                Fp::new(5882809648741838073),
                Fp::new(16439821823239748114),
                Fp::new(15390724433311269161),
                Fp::new(17998117927394775932),
                Fp::new(6759369561378700898),
                Fp::new(17507486457015550303),
                Fp::new(7370721899567647467),
            ],
        ];

        for i in input.iter_mut() {
            apply_sbox(i);
        }

        assert_eq!(input, output);
    }

    #[test]
    fn test_mds() {
        // Hardcoded input / output list generated from the
        // Sagemath code at https://github.com/Nashtare/anemoi-hash/

        let mut input = [
            [Fp::zero(); 8],
            [Fp::one(); 8],
            [
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
                Fp::one(),
                Fp::one(),
                Fp::one(),
                Fp::one(),
            ],
            [
                Fp::one(),
                Fp::one(),
                Fp::one(),
                Fp::one(),
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
                Fp::zero(),
            ],
            [
                Fp::new(17050361427816577515),
                Fp::new(2872212504138397681),
                Fp::new(4695752519446880551),
                Fp::new(4935807951676971936),
                Fp::new(1037171706193426187),
                Fp::new(1190625194406851953),
                Fp::new(4954125988767907363),
                Fp::new(16651299415183992496),
            ],
            [
                Fp::new(17668533851002192536),
                Fp::new(6510390904909121556),
                Fp::new(5908563815378203149),
                Fp::new(5628662287143303977),
                Fp::new(1333978595986847711),
                Fp::new(16737832102436444173),
                Fp::new(3176823622445313606),
                Fp::new(5878838107127395917),
            ],
            [
                Fp::new(592078500502802776),
                Fp::new(15063909079855216842),
                Fp::new(4358653539629520849),
                Fp::new(13335894416572756854),
                Fp::new(15033043095896900581),
                Fp::new(16138621602601198618),
                Fp::new(13254432731627548154),
                Fp::new(2647592412092400882),
            ],
            [
                Fp::new(4588781100086432089),
                Fp::new(17492335098696817332),
                Fp::new(11598242675551189007),
                Fp::new(898199787714285484),
                Fp::new(11475991594161061295),
                Fp::new(9553970465380105433),
                Fp::new(13948585036344341012),
                Fp::new(14529388067828370845),
            ],
            [
                Fp::new(6538970459226487941),
                Fp::new(3631876053100326690),
                Fp::new(3925564046806157520),
                Fp::new(4456051889810703346),
                Fp::new(6211304752369528286),
                Fp::new(16046707403603791081),
                Fp::new(13762975812632989304),
                Fp::new(7386838644880748181),
            ],
            [
                Fp::new(15412025906308720931),
                Fp::new(1414687733709264038),
                Fp::new(6907711019763184428),
                Fp::new(17035963400853502677),
                Fp::new(10021151877764346463),
                Fp::new(13639897022396745097),
                Fp::new(16658858904272065166),
                Fp::new(8380258771263002203),
            ],
        ];

        let mut input2 = input;

        let output = [
            [Fp::zero(); STATE_WIDTH],
            [
                Fp::new(69),
                Fp::new(384),
                Fp::new(321),
                Fp::new(114),
                Fp::new(46),
                Fp::new(256),
                Fp::new(214),
                Fp::new(76),
            ],
            [
                Fp::new(23),
                Fp::new(128),
                Fp::new(107),
                Fp::new(38),
                Fp::new(23),
                Fp::new(128),
                Fp::new(107),
                Fp::new(38),
            ],
            [
                Fp::new(46),
                Fp::new(256),
                Fp::new(214),
                Fp::new(76),
                Fp::new(23),
                Fp::new(128),
                Fp::new(107),
                Fp::new(38),
            ],
            [
                Fp::new(10606019083481807962),
                Fp::new(7144525927766963570),
                Fp::new(5634917752540696674),
                Fp::new(11402706773159628199),
                Fp::new(13837498741178587516),
                Fp::new(6034420467149831011),
                Fp::new(18264246841100978007),
                Fp::new(17814313510811284013),
            ],
            [
                Fp::new(6285000161943817408),
                Fp::new(7590733851542276103),
                Fp::new(14238006718059091695),
                Fp::new(17168494184368782468),
                Fp::new(3346708909334777110),
                Fp::new(10611300784028089610),
                Fp::new(14464684216657215729),
                Fp::new(5369763977968498438),
            ],
            [
                Fp::new(16445886969898868438),
                Fp::new(10217157364557928897),
                Fp::new(17582811750774677825),
                Fp::new(3470579558775592437),
                Fp::new(11288373039527983178),
                Fp::new(9523401967275989379),
                Fp::new(6390582703505603949),
                Fp::new(4512466635227903244),
            ],
            [
                Fp::new(10868919809101485712),
                Fp::new(8482832978431151387),
                Fp::new(9938468860376857999),
                Fp::new(18179480468948542638),
                Fp::new(227289509557205382),
                Fp::new(7521684255485900909),
                Fp::new(15984578307607222751),
                Fp::new(18092533614938489766),
            ],
            [
                Fp::new(4039729038993040766),
                Fp::new(1880342016940610569),
                Fp::new(11098501080008810808),
                Fp::new(1287889868925156853),
                Fp::new(2008158945718834848),
                Fp::new(7029738179710463384),
                Fp::new(8108620823726054910),
                Fp::new(15838062646625466777),
            ],
            [
                Fp::new(308353741438957471),
                Fp::new(13859060998715909008),
                Fp::new(3715914186323394454),
                Fp::new(12451369063064578349),
                Fp::new(8887289784699742032),
                Fp::new(12344271634666009584),
                Fp::new(13688963277823894142),
                Fp::new(15334554711248055697),
            ],
        ];
        for i in input.iter_mut() {
            apply_mds(i);
        }
        for i in input2.iter_mut() {
            apply_naive_mds(i);
        }

        assert_eq!(input, output);
        assert_eq!(input2, output);
    }
}
