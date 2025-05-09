//###################   The CAPSS Framework (2025)   ##################//
//   This file have been ADDED in the context of the implementation    //
// of the CAPSS framework. Please refer to available the change log    //
// for the details.                                                    //
//#####################################################################//

use cheetah::Fp;

use cty;
use std::slice;

use super::{NUM_HASH_ROUNDS,RATE_WIDTH,STATE_WIDTH,NUM_COLUMNS,DIGEST_SIZE,apply_sbox,apply_permutation};
use super::sbox::{ALPHA,BETA,DELTA};
use super::round_constants::{C,D};
use super::hasher::AnemoiHash;
use super::digest::AnemoiDigest;
use crate::traits::Hasher;

/// f64_anemoi_8_get_nb_rounds
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn f64_anemoi_8_get_nb_rounds(nb_rounds: *mut cty::uint32_t) {
    unsafe { *nb_rounds = NUM_HASH_ROUNDS as cty::uint32_t };
}

/// f64_anemoi_8_get_sbox_parameters
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn f64_anemoi_8_get_sbox_parameters(alpha: *mut cty::uint8_t, beta: *mut cty::uint64_t, delta: *mut cty::uint64_t) {
    // Alpha
    unsafe { *alpha = ALPHA as cty::uint8_t };
    // Beta
    if !beta.is_null() {
        unsafe { *beta = BETA.into() };
    }
    // Delta
    if !delta.is_null() {
        unsafe { *delta = DELTA.output_internal() };
    }
}

/// f64_anemoi_8_get_round_keys
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn f64_anemoi_8_get_round_keys(round_keys_ptr: *mut cty::uint64_t) {
    let round_keys_byt: &mut [u64] = unsafe { slice::from_raw_parts_mut(round_keys_ptr, STATE_WIDTH*NUM_HASH_ROUNDS) };
    round_keys_byt.fill(0);
    for i in 0..NUM_HASH_ROUNDS {
        for j in 0..NUM_COLUMNS {
            round_keys_byt[STATE_WIDTH*i+j] = C[i][j].output_internal();
            round_keys_byt[STATE_WIDTH*i+NUM_COLUMNS+j] = D[i][j].output_internal();
        }
    }
}

/// f64_anemoi_8_sbox_layer
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn f64_anemoi_8_sbox_layer(output_ptr: *mut cty::uint64_t, input_ptr: *const cty::uint64_t) {
    let input: &[u64] = unsafe { slice::from_raw_parts(input_ptr, STATE_WIDTH) };
    let output: &mut [u64] = unsafe { slice::from_raw_parts_mut(output_ptr, STATE_WIDTH) };

    let mut i = 0;
    let mut state = [Fp::zero(); STATE_WIDTH];
    for felt in input {
        state[i] = Fp::new(*felt);
        i += 1;
    }
    apply_sbox(&mut state);
    i = 0;
    for felt in state {
        output[i] = felt.output_internal();
        i += 1;
    }
}

/// f64_compress2
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn f64_compress2(output_ptr: *mut cty::uint64_t, input_ptr: *const cty::uint64_t) {
    let input_raw: &[u64] = unsafe { slice::from_raw_parts(input_ptr, STATE_WIDTH) };
    let output_raw: &mut [u64] = unsafe { slice::from_raw_parts_mut(output_ptr, DIGEST_SIZE) };

    let mut input = [AnemoiDigest::default(); 2];
    input[0] = AnemoiDigest::new([
        Fp::new(input_raw[0]),
        Fp::new(input_raw[1]),
        Fp::new(input_raw[2]),
        Fp::new(input_raw[3]),
    ]);
    input[1] = AnemoiDigest::new([
        Fp::new(input_raw[4]),
        Fp::new(input_raw[5]),
        Fp::new(input_raw[6]),
        Fp::new(input_raw[7]),
    ]);
    
    let binding = AnemoiHash::merge(&input);
    let output = binding.as_elements();
    output_raw[0] = output[0].output_internal();
    output_raw[1] = output[1].output_internal();
    output_raw[2] = output[2].output_internal();
    output_raw[3] = output[3].output_internal();
}

fn anemoi_xof_field(elems: &[Fp], out_size: usize) -> Vec<Fp> {
    // initialize state to all zeros
    let mut state = [Fp::zero(); STATE_WIDTH];

    let sigma = if elems.len() % RATE_WIDTH == 0 {
        Fp::one()
    } else {
        Fp::zero()
    };

    let mut i = 0;
    for &element in elems.iter() {
        state[i] += element;
        i += 1;
        if i % RATE_WIDTH == 0 {
            apply_permutation(&mut state);
            i = 0;
        }
    }

    // We then add sigma to the last register of the capacity.
    state[STATE_WIDTH - 1] += sigma;

    // If the message length is not a multiple of RATE_WIDTH, we append 1 to the rate cell
    // next to the one where we previously appended the last message element. This is
    // guaranted to be in the rate registers (i.e. to not require an extra permutation before
    // adding this constant) if sigma is equal to zero. We then apply a final Anemoi permutation
    // to the whole state.
    if sigma.is_zero().into() {
        state[i] += Fp::one();
        apply_permutation(&mut state);
    }

    // Squeezing phase
    let mut res = Vec::with_capacity(out_size);
    i = 0;
    for _k in 0..out_size {
        if (i > 0) && (i % RATE_WIDTH == 0) {
            apply_permutation(&mut state);
            i = 0;
        }
        res.push(state[i]);
        i += 1;
    }

    res
}

/// f64_xof
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn f64_xof(output_ptr: *mut cty::uint64_t, input_ptr: *const cty::uint64_t, input_size: cty::uint32_t, output_size: cty::uint32_t) {
    let input_raw: &[u64] = unsafe { slice::from_raw_parts(input_ptr, (input_size).try_into().unwrap()) };
    let output_raw: &mut [u64] = unsafe { slice::from_raw_parts_mut(output_ptr, (output_size).try_into().unwrap()) };

    let mut input: Vec<Fp> = Vec::with_capacity((input_size).try_into().unwrap());
    for felt in input_raw {
        input.push(Fp::new(*felt));
    }
    let output = anemoi_xof_field(&input, output_size.try_into().unwrap());
    let mut i = 0;
    for felt in output {
        output_raw[i] = felt.output_internal();
        i += 1;
    }
}
