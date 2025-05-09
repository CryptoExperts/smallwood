// Copyright (c) 2021-2022 Toposware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::STATE_WIDTH;

/// Maximum Diffusion Layer matrix for Griffin.
#[allow(unused)]
pub(crate) const MDS: [u64; STATE_WIDTH * STATE_WIDTH] = [
    10, 14, 2, 6, 5, 7, 1, 3, 5, 7, 1, 3, 8, 12, 2, 2, 4, 6, 1, 1, 4, 6, 1, 1, 2, 6, 10, 14, 1, 3,
    5, 7, 1, 3, 5, 7, 2, 2, 8, 12, 1, 1, 4, 6, 1, 1, 4, 6, 5, 7, 1, 3, 10, 14, 2, 6, 5, 7, 1, 3, 4,
    6, 1, 1, 8, 12, 2, 2, 4, 6, 1, 1, 1, 3, 5, 7, 2, 6, 10, 14, 1, 3, 5, 7, 1, 1, 4, 6, 2, 2, 8,
    12, 1, 1, 4, 6, 5, 7, 1, 3, 5, 7, 1, 3, 10, 14, 2, 6, 4, 6, 1, 1, 4, 6, 1, 1, 8, 12, 2, 2, 1,
    3, 5, 7, 1, 3, 5, 7, 2, 6, 10, 14, 1, 1, 4, 6, 1, 1, 4, 6, 2, 2, 8, 12,
];
