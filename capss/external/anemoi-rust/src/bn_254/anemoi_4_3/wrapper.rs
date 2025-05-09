//###################   The CAPSS Framework (2025)   ##################//
//   This file have been ADDED in the context of the implementation    //
// of the CAPSS framework. Please refer to available the change log    //
// for the details.                                                    //
//#####################################################################//

use num_bigint::BigUint;
use cty;
use std::slice;
use ark_ff::Field;
use ark_ff::PrimeField;

use super::Felt;
use super::AnemoiBn254_4_3;
use super::{RATE_WIDTH,STATE_WIDTH};
use super::{Anemoi, Jive};
use super::{One, Zero};

/// bn254_field_reduce
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_field_reduce(c_ptr: *mut cty::uint8_t, a_ptr: *const cty::uint8_t) {
    let a_byt: &[u8] = unsafe { slice::from_raw_parts(a_ptr, 32) };
    let c_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(c_ptr, 32) };
    let a = Felt::from(BigUint::from_bytes_le(&a_byt));
    let res: BigUint = (a).into();
    let res_slice = res.to_bytes_le();
    c_byt.fill(0);
    c_byt[0..res_slice.len()].clone_from_slice(&res_slice);
}

/// bn254_field_add
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_field_add(c_ptr: *mut cty::uint8_t, a_ptr: *const cty::uint8_t, b_ptr: *const cty::uint8_t) {
    let a_byt: &[u8] = unsafe { slice::from_raw_parts(a_ptr, 32) };
    let b_byt: &[u8] = unsafe { slice::from_raw_parts(b_ptr, 32) };
    let c_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(c_ptr, 32) };
    let a = Felt::from(BigUint::from_bytes_le(&a_byt));
    let b = Felt::from(BigUint::from_bytes_le(&b_byt));
    let res: BigUint = (a+b).into();
    let res_slice = res.to_bytes_le();
    c_byt.fill(0);
    c_byt[0..res_slice.len()].clone_from_slice(&res_slice);
}

/// bn254_field_neg
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_field_neg(c_ptr: *mut cty::uint8_t, a_ptr: *const cty::uint8_t) {
    let a_byt: &[u8] = unsafe { slice::from_raw_parts(a_ptr, 32) };
    let c_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(c_ptr, 32) };
    let a = Felt::from(BigUint::from_bytes_le(&a_byt));
    let res: BigUint = (-a).into();
    let res_slice = res.to_bytes_le();
    c_byt.fill(0);
    c_byt[0..res_slice.len()].clone_from_slice(&res_slice);
}

/// bn254_field_sub
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_field_sub(c_ptr: *mut cty::uint8_t, a_ptr: *const cty::uint8_t, b_ptr: *const cty::uint8_t) {
    let a_byt: &[u8] = unsafe { slice::from_raw_parts(a_ptr, 32) };
    let b_byt: &[u8] = unsafe { slice::from_raw_parts(b_ptr, 32) };
    let c_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(c_ptr, 32) };
    let a = Felt::from(BigUint::from_bytes_le(&a_byt));
    let b = Felt::from(BigUint::from_bytes_le(&b_byt));
    let res: BigUint = (a-b).into();
    let res_slice = res.to_bytes_le();
    c_byt.fill(0);
    c_byt[0..res_slice.len()].clone_from_slice(&res_slice);
}

/// bn254_field_mul
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_field_mul(c_ptr: *mut cty::uint8_t, a_ptr: *const cty::uint8_t, b_ptr: *const cty::uint8_t) {
    let a_byt: &[u8] = unsafe { slice::from_raw_parts(a_ptr, 32) };
    let b_byt: &[u8] = unsafe { slice::from_raw_parts(b_ptr, 32) };
    let c_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(c_ptr, 32) };
    let a = Felt::from(BigUint::from_bytes_le(&a_byt));
    let b = Felt::from(BigUint::from_bytes_le(&b_byt));
    let res: BigUint = (a*b).into();
    let res_slice = res.to_bytes_le();
    c_byt.fill(0);
    c_byt[0..res_slice.len()].clone_from_slice(&res_slice);
}

/// bn254_field_inv
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_field_inv(c_ptr: *mut cty::uint8_t, a_ptr: *const cty::uint8_t) {
    let a_byt: &[u8] = unsafe { slice::from_raw_parts(a_ptr, 32) };
    let c_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(c_ptr, 32) };
    let a = Felt::from(BigUint::from_bytes_le(&a_byt));
    let res: BigUint = (a.inverse().unwrap()).into();
    let res_slice = res.to_bytes_le();
    c_byt.fill(0);
    c_byt[0..res_slice.len()].clone_from_slice(&res_slice);
}

/// bn254_field_div
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_field_div(c_ptr: *mut cty::uint8_t, a_ptr: *const cty::uint8_t, b_ptr: *const cty::uint8_t) {
    let a_byt: &[u8] = unsafe { slice::from_raw_parts(a_ptr, 32) };
    let b_byt: &[u8] = unsafe { slice::from_raw_parts(b_ptr, 32) };
    let c_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(c_ptr, 32) };
    let a = Felt::from(BigUint::from_bytes_le(&a_byt));
    let b = Felt::from(BigUint::from_bytes_le(&b_byt));
    let res: BigUint = (a*(b.inverse().unwrap())).into();
    let res_slice = res.to_bytes_le();
    c_byt.fill(0);
    c_byt[0..res_slice.len()].clone_from_slice(&res_slice);
}

/// bn254_field_is_equal
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_field_is_equal(a_ptr: *const cty::uint8_t, b_ptr: *const cty::uint8_t) -> cty::c_int {
    let a_byt: &[u8] = unsafe { slice::from_raw_parts(a_ptr, 32) };
    let b_byt: &[u8] = unsafe { slice::from_raw_parts(b_ptr, 32) };
    let a = Felt::from(BigUint::from_bytes_le(&a_byt));
    let b = Felt::from(BigUint::from_bytes_le(&b_byt));
    return if a == b { 1 } else { 0 };
}

/// bn254_compress4
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_compress4(output_ptr: *mut cty::uint8_t, input_ptr: *const cty::uint8_t) {
    let input_byt: &[u8] = unsafe { slice::from_raw_parts(input_ptr, 4*32) };
    let output_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(output_ptr, 1*32) };
    let mut input: Vec<Felt> = Vec::with_capacity(4);
    for chunk in input_byt.chunks(32) {
        input.push(Felt::from_le_bytes_mod_order(&chunk[..]));
    }
    let output = AnemoiBn254_4_3::compress(&input.into_boxed_slice());
    let res: BigUint = output[0].into();
    let res_slice = res.to_bytes_le();
    output_byt.fill(0);
    output_byt[0..res_slice.len()].clone_from_slice(&res_slice);
}

fn anemoi_xof_field(elems: &[Felt], out_size: usize) -> Vec<Felt> {
    // initialize state to all zeros
    let mut state = [Felt::zero(); STATE_WIDTH];

    let sigma = if elems.len() % RATE_WIDTH == 0 {
        Felt::one()
    } else {
        Felt::zero()
    };

    let mut i = 0;
    for &element in elems.iter() {
        state[i] += element;
        i += 1;
        if i % RATE_WIDTH == 0 {
            AnemoiBn254_4_3::permutation(&mut state);
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
    if sigma.is_zero() {
        state[i] += Felt::one();
        AnemoiBn254_4_3::permutation(&mut state);
    }

    // Squeezing phase
    let mut res = Vec::with_capacity(out_size);
    i = 0;
    for _k in 0..out_size {
        if (i > 0) && (i % RATE_WIDTH == 0) {
            AnemoiBn254_4_3::permutation(&mut state);
            i = 0;
        }
        res.push(state[i]);
        i += 1;
    }

    res
}

/// bn254_xof
#[no_mangle]
#[allow(unsafe_code)]
pub extern "C" fn bn254_xof(output_ptr: *mut cty::uint8_t, input_ptr: *const cty::uint8_t, input_felt_size: cty::uint32_t, output_felt_size: cty::uint32_t) {
    let input_byt: &[u8] = unsafe { slice::from_raw_parts(input_ptr, (32*input_felt_size).try_into().unwrap()) };
    let output_byt: &mut [u8] = unsafe { slice::from_raw_parts_mut(output_ptr, (32*output_felt_size).try_into().unwrap()) };
    let mut input: Vec<Felt> = Vec::with_capacity(input_felt_size.try_into().unwrap());
    for chunk in input_byt.chunks(32) {
        input.push(Felt::from_le_bytes_mod_order(&chunk[..]));
    }
    let output = anemoi_xof_field(&input.into_boxed_slice(), output_felt_size.try_into().unwrap());
    let mut i = 0;
    output_byt.fill(0);
    for felt in output {
        let res: BigUint = felt.into();
        let res_slice = res.to_bytes_le();
        output_byt[(32*i)..(32*i+res_slice.len())].clone_from_slice(&res_slice);
        i += 1;
    }
}
