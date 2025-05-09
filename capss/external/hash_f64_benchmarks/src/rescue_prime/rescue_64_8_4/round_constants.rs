// Copyright (c) 2021-2023 Toposware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{NUM_HASH_ROUNDS, STATE_WIDTH};
use cheetah::Fp;

/// Additive Round Keys constants for Rescue,
/// computed using algorithm 5 from <https://eprint.iacr.org/2020/1143.pdf>
pub(crate) const ARK: [[Fp; STATE_WIDTH * 2]; NUM_HASH_ROUNDS] = [
    [
        Fp::new(5250156239823432273),
        Fp::new(17991370199276831394),
        Fp::new(15363758995121189373),
        Fp::new(7550390719632034712),
        Fp::new(705744964663370588),
        Fp::new(14718080047998507086),
        Fp::new(15612952641514293932),
        Fp::new(8827614218997241655),
        Fp::new(9860068499471230379),
        Fp::new(10391494434594667033),
        Fp::new(4986587677027284267),
        Fp::new(17781977240739864050),
        Fp::new(6888921375142581299),
        Fp::new(8950831725295674725),
        Fp::new(17048848277806802259),
        Fp::new(14146306451370933851),
    ],
    [
        Fp::new(3820104553051581938),
        Fp::new(3385456123263281593),
        Fp::new(16094709323995557719),
        Fp::new(16303336019291352506),
        Fp::new(8678496957982514796),
        Fp::new(498270172890916765),
        Fp::new(17676155962043649331),
        Fp::new(14993644560894569061),
        Fp::new(707569561928852298),
        Fp::new(6724851263229096394),
        Fp::new(16052786826295129381),
        Fp::new(1966016718617096590),
        Fp::new(9416027981257317341),
        Fp::new(650995073054283087),
        Fp::new(10013853213448688130),
        Fp::new(14400137552134409897),
    ],
    [
        Fp::new(14258773164148374760),
        Fp::new(1655972393090532756),
        Fp::new(7105012644980738960),
        Fp::new(11852376844296856307),
        Fp::new(17816158174482938174),
        Fp::new(3981864273667206359),
        Fp::new(2807469273751819673),
        Fp::new(14974221859211617968),
        Fp::new(7149263702162640230),
        Fp::new(7096225564191267298),
        Fp::new(12197502430442379401),
        Fp::new(12804378092281676880),
        Fp::new(17409570408925731570),
        Fp::new(2819914464281065415),
        Fp::new(15831648359524824910),
        Fp::new(15629743966484525526),
    ],
    [
        Fp::new(15947271309323471269),
        Fp::new(14698197888879866148),
        Fp::new(14077077040726269118),
        Fp::new(2859805440338816615),
        Fp::new(4945184696648790387),
        Fp::new(15183288803792940883),
        Fp::new(7601775560447886378),
        Fp::new(6477224812816853098),
        Fp::new(17953398529773387863),
        Fp::new(6198711330432012203),
        Fp::new(9157726872360640492),
        Fp::new(9493333679697066249),
        Fp::new(16030612341681265024),
        Fp::new(4739709630031417239),
        Fp::new(18287301685877696586),
        Fp::new(8798230489526342293),
    ],
    [
        Fp::new(18213733347601447845),
        Fp::new(10031679943792626621),
        Fp::new(5971928707867502549),
        Fp::new(4916840084933933812),
        Fp::new(3613815642787339926),
        Fp::new(16715066477165606893),
        Fp::new(14603075385258290966),
        Fp::new(6037771699330759024),
        Fp::new(11624786627634502148),
        Fp::new(12924370583547723043),
        Fp::new(11192385058160295505),
        Fp::new(14350900531623057057),
        Fp::new(6649040255431543914),
        Fp::new(2106567763792008889),
        Fp::new(12434281915569617273),
        Fp::new(8101377239551798417),
    ],
    [
        Fp::new(11092469678405138663),
        Fp::new(14512788091784891767),
        Fp::new(12690682422447262976),
        Fp::new(4807355108863118656),
        Fp::new(5207405791308193025),
        Fp::new(5970889292753030887),
        Fp::new(17691092604759176390),
        Fp::new(2731892623388788619),
        Fp::new(13925815041351874730),
        Fp::new(15981136477777934021),
        Fp::new(17398194123970783302),
        Fp::new(17377636820017036987),
        Fp::new(5173992930377549692),
        Fp::new(3688194845376511083),
        Fp::new(16177005022792194790),
        Fp::new(6482787365501773067),
    ],
    [
        Fp::new(9320990164295747317),
        Fp::new(8313044787501051613),
        Fp::new(15388579942433649113),
        Fp::new(16827303822369113172),
        Fp::new(7362247368635881413),
        Fp::new(5501558211335089067),
        Fp::new(16959364163466644433),
        Fp::new(15127897185888596873),
        Fp::new(9197066592623932055),
        Fp::new(1777435748159421921),
        Fp::new(5079482957444239813),
        Fp::new(15080163201683705054),
        Fp::new(4278835591662809119),
        Fp::new(6609842793229774583),
        Fp::new(651644751771720476),
        Fp::new(14434199410773467460),
    ],
    [
        Fp::new(17649298456157115580),
        Fp::new(943073344101973600),
        Fp::new(18143641070681943751),
        Fp::new(10281169186835896355),
        Fp::new(13406813280987837753),
        Fp::new(15829668250171815180),
        Fp::new(18110012633957611059),
        Fp::new(1006437581482348697),
        Fp::new(18260631255173088400),
        Fp::new(11878555248395841583),
        Fp::new(656803708484073383),
        Fp::new(4476884818921019435),
        Fp::new(9874547310377130300),
        Fp::new(5749523373938683929),
        Fp::new(6197326326877598397),
        Fp::new(16829013309168536734),
    ],
];
