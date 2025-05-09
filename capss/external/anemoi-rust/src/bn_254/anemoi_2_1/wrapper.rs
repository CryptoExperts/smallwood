//###################   The CAPSS Framework (2025)   ##################//
//   This file have been ADDED in the context of the implementation    //
// of the CAPSS framework. Please refer to available the change log    //
// for the details.                                                    //
//#####################################################################//

use num_bigint::BigUint;
use cty;
use std::slice;
use ark_ff::PrimeField;

use super::Felt;
use super::AnemoiBn254_2_1;
use super::{Anemoi, Jive};
use super::{Zero};

/// bn254_compress2
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_compress2(output_ptr: *mut cty::uint8_t, input_ptr: *const cty::uint8_t) {
    let input_byt: &[u8] = unsafe { slice::from_raw_parts(input_ptr, 2*32) };
    let output_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(output_ptr, 1*32) };
    let mut input: Vec<Felt> = Vec::with_capacity(2);
    for chunk in input_byt.chunks(32) {
        input.push(Felt::from_le_bytes_mod_order(&chunk[..]));
    }
    let output = AnemoiBn254_2_1::compress(&input.into_boxed_slice());
    let res: BigUint = output[0].into();
    let res_slice = res.to_bytes_le();
    output_byt.fill(0);
    output_byt[0..res_slice.len()].clone_from_slice(&res_slice);
}

/// bn254_anemoi_2_get_nb_rounds
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_anemoi_2_get_nb_rounds(nb_rounds: *mut cty::uint32_t) {
    unsafe { *nb_rounds = AnemoiBn254_2_1::NUM_ROUNDS as cty::uint32_t };
}

/// bn254_anemoi_2_get_sbox_parameters
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_anemoi_2_get_sbox_parameters(alpha: *mut cty::uint8_t, beta_ptr: *mut cty::uint8_t, delta_ptr: *mut cty::uint8_t) {
    // Alpha
    unsafe { *alpha = AnemoiBn254_2_1::ALPHA as cty::uint8_t };
    // Beta
    if !beta_ptr.is_null() {
        let beta_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(beta_ptr, 32) };
        let beta_biguint: BigUint = AnemoiBn254_2_1::BETA.into();
        let beta_slice = beta_biguint.to_bytes_le();
        beta_byt.fill(0);
        beta_byt[..beta_slice.len()].clone_from_slice(&beta_slice);
    }
    // Delta
    if !delta_ptr.is_null() {
        let delta_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(delta_ptr, 32) };
        let delta_biguint: BigUint = AnemoiBn254_2_1::DELTA.into();
        let delta_slice = delta_biguint.to_bytes_le();
        delta_byt.fill(0);
        delta_byt[..delta_slice.len()].clone_from_slice(&delta_slice);
    }
}

/// bn254_anemoi_2_get_round_keys
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_anemoi_2_get_round_keys(round_keys_ptr: *mut cty::uint8_t) {
    let round_keys_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(round_keys_ptr, 32*AnemoiBn254_2_1::WIDTH*AnemoiBn254_2_1::NUM_ROUNDS) };
    round_keys_byt.fill(0);
    for i in 0..AnemoiBn254_2_1::NUM_ROUNDS {
        let res_c: BigUint = AnemoiBn254_2_1::ARK_C[i].into();
        let res_d: BigUint = AnemoiBn254_2_1::ARK_D[i].into();
        let res_c_slice = res_c.to_bytes_le();
        let res_d_slice = res_d.to_bytes_le();
        round_keys_byt[(32*2*i)..(32*2*i+res_c_slice.len())].clone_from_slice(&res_c_slice);
        round_keys_byt[(32*(2*i+1))..(32*(2*i+1)+res_d_slice.len())].clone_from_slice(&res_d_slice);
    }
}

/// bn254_anemoi_2_sbox_layer
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_anemoi_2_sbox_layer(output_ptr: *mut cty::uint8_t, input_ptr: *const cty::uint8_t) {
    let input_byt: &[u8] = unsafe { slice::from_raw_parts(input_ptr, 32*AnemoiBn254_2_1::WIDTH) };
    let output_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(output_ptr, 32*AnemoiBn254_2_1::WIDTH) };
    output_byt.fill(0);

    let mut i = 0;
    let mut state = [Felt::zero(); AnemoiBn254_2_1::WIDTH];
    for chunk in input_byt.chunks(32) {
        state[i] = Felt::from_le_bytes_mod_order(&chunk[..]);
        i += 1;
    }
    AnemoiBn254_2_1::sbox_layer(&mut state);
    i = 0;
    for felt in state {
        let res: BigUint = felt.into();
        let res_slice = res.to_bytes_le();
        output_byt[(32*i)..(32*i+res_slice.len())].clone_from_slice(&res_slice);
        i += 1;
    }
}
