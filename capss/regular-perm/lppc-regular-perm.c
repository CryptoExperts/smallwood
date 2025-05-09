#include "lppc-regular-perm.h"
#include "permutation-round.h"
#include "utils.h"
#include <stdio.h>

struct reg_perm_lppc_cfg_t {
    lppc_cfg_t main;
    uint32_t state_size; // t
    uint32_t nb_rounds; // n_r
    uint32_t batching_factor; // b
    uint32_t witness_size; // |v|
    uint32_t iv_size; // |iv|
    uint32_t y_size; // |y|
};

struct reg_perm_lppc_t {
    reg_perm_lppc_cfg_t lppc_cfg;
    vec_t iv;
    vec_t y;
};

uint32_t rperm_lppc_get_preprocessing_material_bytesize(const reg_perm_lppc_cfg_t* lppc_cfg);
void rperm_lppc_preprocess_packing_points(const reg_perm_lppc_cfg_t* lppc_cfg, const vec_t packing_points, uint8_t* preprocessing_material);

void rperm_lppc_get_constraint_pol_polynomials(const reg_perm_lppc_t* lppc, const poly_t* wit_polys, const uint8_t* preprocessing_material, poly_t* in_ppol, uint32_t wit_poly_degree);
void rperm_lppc_get_constraint_lin_polynomials(const reg_perm_lppc_t* lppc, const poly_t* wit_polys, const uint8_t* preprocessing_material, poly_t* in_plin, uint32_t wit_poly_degree);
void rperm_lppc_get_constraint_lin_polynomials_batched(const reg_perm_lppc_t* lppc, const poly_t* wit_polys, const uint8_t* preprocessing_material, const vec_t* gammas, poly_t* in_plin, uint32_t wit_poly_degree, uint32_t rho);
void rperm_lppc_get_linear_result(const reg_perm_lppc_t* lppc, vec_t vt);

void rperm_lppc_get_constraint_pol_evals(const reg_perm_lppc_t* lppc, const vec_t eval_points, const vec_t* evals, const uint8_t* preprocessing_material, uint32_t nb_evals, vec_t* in_epol);
void rperm_lppc_get_constraint_lin_evals(const reg_perm_lppc_t* lppc, const vec_t eval_points, const vec_t* evals, const uint8_t* preprocessing_material, uint32_t nb_evals, vec_t* in_elin);

reg_perm_lppc_cfg_t* rperm_lppc_malloc_config(uint32_t batching_factor, uint32_t iv_size, uint32_t y_size) {
    uint32_t state_size, nb_rounds, witness_size, verification_degree;
    get_permutation_parameters(&state_size, &nb_rounds, &witness_size, &verification_degree);

    if(batching_factor == 0) {
        fprintf(stderr, "Regular Perm LPPC: the batching factor (%d) should be non-zero\n", batching_factor);
        return NULL;
    }

    uint32_t packing_factor = (nb_rounds + batching_factor - 1) / batching_factor;
    if(iv_size == 0 || y_size == 0) {
        fprintf(stderr, "Regular Perm LPPC: the IV size (%d) and the public output size (%d) should be non-zero\n", iv_size, y_size);
        return NULL;
    }
    if(iv_size >= state_size) {
        fprintf(stderr, "Regular Perm LPPC: the IV size (%d) should be (strictly) smaller than the state size (%d)\n", iv_size, state_size);
        return NULL;
    }
    if(y_size > state_size) {
        fprintf(stderr, "Regular Perm LPPC: the public output size (%d) should be smaller than (or equal to) the state size (%d)\n", y_size, state_size);
        return NULL;
    }

    reg_perm_lppc_cfg_t* lppc_cfg = malloc(sizeof(reg_perm_lppc_cfg_t));
    
    lppc_cfg->main.nb_wit_rows = state_size*(batching_factor+1) + witness_size*batching_factor;
    lppc_cfg->main.packing_factor = packing_factor;
    lppc_cfg->main.constraint_degree = verification_degree;
    lppc_cfg->main.nb_poly_constraints = batching_factor*(state_size+witness_size);
    lppc_cfg->main.nb_linear_constraints = state_size*(packing_factor-1) + iv_size + y_size;
    lppc_cfg->main.get_preprocessing_material_bytesize = (uint32_t (*)(const lppc_cfg_t*)) rperm_lppc_get_preprocessing_material_bytesize;
    lppc_cfg->main.preprocess_packing_points = (void (*)(const lppc_cfg_t*, const vec_t, uint8_t*)) rperm_lppc_preprocess_packing_points;
    lppc_cfg->main.get_constraint_pol_polynomials = (void (*)(const lppc_t*, const poly_t*, const uint8_t*, poly_t*, uint32_t)) rperm_lppc_get_constraint_pol_polynomials;
    lppc_cfg->main.get_constraint_lin_polynomials = (void (*)(const lppc_t*, const poly_t*, const uint8_t*, poly_t*, uint32_t)) rperm_lppc_get_constraint_lin_polynomials;
    lppc_cfg->main.get_constraint_lin_polynomials_batched = (void (*)(const lppc_t* lppc, const poly_t*, const uint8_t*, const vec_t*, poly_t*, uint32_t, uint32_t)) rperm_lppc_get_constraint_lin_polynomials_batched;
    lppc_cfg->main.get_linear_result = (void (*)(const lppc_t*, vec_t)) rperm_lppc_get_linear_result;
    lppc_cfg->main.get_constraint_pol_evals = (void (*)(const lppc_t*, const vec_t, const vec_t*, const uint8_t*, uint32_t, vec_t*)) rperm_lppc_get_constraint_pol_evals;
    lppc_cfg->main.get_constraint_lin_evals = (void (*)(const lppc_t*, const vec_t, const vec_t*, const uint8_t*, uint32_t, vec_t*)) rperm_lppc_get_constraint_lin_evals;

    lppc_cfg->state_size = state_size;
    lppc_cfg->nb_rounds = nb_rounds;
    lppc_cfg->batching_factor = batching_factor;
    lppc_cfg->witness_size = witness_size;
    lppc_cfg->iv_size = iv_size;
    lppc_cfg->y_size = y_size;

    return lppc_cfg;
}

const lppc_cfg_t* rperm_lppc_get_config_lppc(const reg_perm_lppc_cfg_t* lppc_cfg) {
    if(lppc_cfg == NULL)
        return NULL;
    return &lppc_cfg->main;
}

const reg_perm_lppc_cfg_t* rperm_lppc_get_config(const reg_perm_lppc_t* lppc) {
    if(lppc == NULL)
        return NULL;
    return &lppc->lppc_cfg;
}

reg_perm_lppc_t* rperm_lppc_malloc(const reg_perm_lppc_cfg_t* lppc_cfg, const vec_t iv, const vec_t y) {
    if(lppc_cfg == NULL)
        return NULL;
        
    uint32_t iv_size = lppc_cfg->iv_size;
    uint32_t y_size = lppc_cfg->y_size;

    uint32_t lppc_bytesize = sizeof(reg_perm_lppc_t);
    uint32_t iv_bytesize = sizeof(felt_t)*iv_size;
    uint32_t y_bytesize = sizeof(felt_t)*y_size;
    uint32_t total_bytesize = lppc_bytesize + iv_bytesize + y_bytesize;
    uint8_t* ptr = malloc(total_bytesize);
    if(ptr == NULL)
        return NULL;
    reg_perm_lppc_t* lppc = (reg_perm_lppc_t*) ptr;
    lppc->iv = (vec_t) (ptr + lppc_bytesize);
    lppc->y = (vec_t) (ptr + lppc_bytesize + iv_bytesize);
    memcpy(&lppc->lppc_cfg, lppc_cfg, sizeof(reg_perm_lppc_cfg_t));
    vec_set(lppc->iv, iv, iv_size);
    vec_set(lppc->y, y, y_size);
    return lppc;
}

int rperm_lppc_random(const reg_perm_lppc_cfg_t* lppc_cfg, reg_perm_lppc_t** lppc, vec_t* secret) {
    if(lppc_cfg == NULL)
        return -1;

    uint32_t state_size = lppc_cfg->state_size;
    uint32_t nb_rounds = lppc_cfg->nb_rounds;
    uint32_t iv_size = lppc_cfg->iv_size;
    uint32_t y_size = lppc_cfg->y_size;
    
    vec_t iv = malloc_vec(iv_size);
    vec_t x = malloc_vec(state_size-iv_size);
    vec_t round_keys = malloc_vec(nb_rounds*state_size);
    vec_t state = malloc_vec(state_size);
    vec_t y = malloc_vec(y_size);
    if(iv == NULL || x == NULL || y == NULL || round_keys == NULL || state == NULL) {
        free(iv);
        free(x);
        free(round_keys);
        free(state);
        free(y);
        return -1;
    }

    vec_random(iv, iv_size);
    vec_random(x, state_size-iv_size);

    get_round_keys(round_keys);

    vec_set(state, iv, iv_size);
    vec_set(&state[iv_size], x, state_size-iv_size);
    for(uint32_t num_round=0; num_round<nb_rounds; num_round++) {
        vec_t round_cst = &round_keys[num_round*state_size];
        compute_round_function_with_witness(state, round_cst, state_size, state, NULL);
    }
    vec_set(y, state, y_size);
    free(state);
    free(round_keys);

    *secret = x;
    *lppc = rperm_lppc_malloc(lppc_cfg, iv, y);
    free(iv);
    free(y);
    return 0;
}

int rperm_lppc_build_witness(const reg_perm_lppc_t* lppc, const vec_t secret, vec_t witness) {
    const vec_t x = secret;
    
    const reg_perm_lppc_cfg_t* lppc_cfg = rperm_lppc_get_config(lppc);
    uint32_t packing_factor = lppc_cfg->main.packing_factor;
    uint32_t state_size = lppc_cfg->state_size;
    uint32_t nb_rounds = lppc_cfg->nb_rounds;
    uint32_t batching_factor = lppc_cfg->batching_factor;
    uint32_t witness_size = lppc_cfg->witness_size;
    uint32_t iv_size = lppc_cfg->iv_size;

    vec_t iv = lppc->iv;

    vec_t state = malloc_vec(state_size);
    vec_t round_wit = witness_size ? malloc_vec(witness_size) : NULL;
    vec_set(state, iv, iv_size);
    vec_set(&state[iv_size], x, state_size-iv_size);

    vec_t round_keys = malloc_vec(nb_rounds*state_size);
    get_round_keys(round_keys);

    for(uint32_t num_round=0; num_round<batching_factor*packing_factor; num_round++) {
        uint32_t num_col = num_round / batching_factor;

        if(num_round % batching_factor == 0) {
            for(uint32_t i=0; i<state_size; i++)
                felt_set(&witness[i*packing_factor+num_col], &state[i]);
        }

        // We do not care of the values of round_cst after "nb_rounds" rounds
        vec_t round_cst = (num_round < nb_rounds) ? &round_keys[num_round*state_size] : &round_keys[0];
        compute_round_function_with_witness(state, round_cst, state_size, state, round_wit);

        uint32_t offset_row = ((num_round % batching_factor) + 1)*state_size;
        for(uint32_t i=0; i<state_size; i++)
            felt_set(&witness[(offset_row+i)*packing_factor+num_col], &state[i]);
        
        offset_row = (batching_factor + 1)*state_size + (num_round % batching_factor)*witness_size;
        for(uint32_t i=0; i<witness_size; i++)
            felt_set(&witness[(offset_row+i)*packing_factor+num_col], &round_wit[i]);
    }
    free(state);
    if(round_wit)
        free(round_wit);
    free(round_keys);
    return 0;
}

uint32_t rperm_lppc_get_secret_size(const reg_perm_lppc_cfg_t* lppc_cfg) {
    uint32_t state_size = lppc_cfg->state_size;
    uint32_t iv_size = lppc_cfg->iv_size;
    return state_size-iv_size;
}


uint32_t rperm_lppc_get_serialized_bytesize(const reg_perm_lppc_cfg_t* lppc_cfg) {
    uint32_t iv_size = lppc_cfg->iv_size;
    uint32_t y_size = lppc_cfg->y_size;
    return vec_get_bytesize(iv_size) + vec_get_bytesize(y_size);
}

void rperm_lppc_serialize(uint8_t* buffer, const reg_perm_lppc_t* lppc) {
    const reg_perm_lppc_cfg_t* lppc_cfg = rperm_lppc_get_config(lppc);
    uint32_t iv_size = lppc_cfg->iv_size;
    uint32_t y_size = lppc_cfg->y_size;
    WRITE_BUFFER_VEC(buffer, lppc->iv, iv_size);
    WRITE_BUFFER_VEC(buffer, lppc->y, y_size);
}

reg_perm_lppc_t* rperm_lppc_malloc_from_serialized(const reg_perm_lppc_cfg_t* lppc_cfg, const uint8_t* buffer) {
    uint32_t iv_size = lppc_cfg->iv_size;
    uint32_t y_size = lppc_cfg->y_size;
    
    vec_t iv = malloc_vec(iv_size);
    vec_t y = malloc_vec(y_size);
    if(iv == NULL || y == NULL) {
        free(iv);
        free(y);
        return NULL;
    }
    READ_BUFFER_VEC(iv, buffer, iv_size);
    READ_BUFFER_VEC(y, buffer, y_size);
    reg_perm_lppc_t* lppc = rperm_lppc_malloc(lppc_cfg, iv, y);
    free(iv);
    free(y);
    return lppc;
}

uint32_t rperm_lppc_get_preprocessing_material_bytesize(const reg_perm_lppc_cfg_t* lppc_cfg) {
    return sizeof(felt_t)*lppc_cfg->main.packing_factor;
}

void rperm_lppc_preprocess_packing_points(const reg_perm_lppc_cfg_t* lppc_cfg, const vec_t packing_points, uint8_t* preprocessing_material) {
    uint32_t packing_factor = lppc_cfg->main.packing_factor;
    memcpy(preprocessing_material, (uint8_t*) packing_points, sizeof(felt_t)*packing_factor);
}

void rperm_lppc_get_constraint_pol_polynomials(const reg_perm_lppc_t* lppc, const poly_t* wit_polys, const uint8_t* preprocessing_material, poly_t* in_ppol, uint32_t wit_poly_degree) {
    vec_t packing_points = (vec_t) preprocessing_material;

    const reg_perm_lppc_cfg_t* lppc_cfg = rperm_lppc_get_config(lppc);
    uint32_t packing_factor = lppc_cfg->main.packing_factor;
    uint32_t state_size = lppc_cfg->state_size;
    uint32_t nb_rounds = lppc_cfg->nb_rounds;
    uint32_t batching_factor = lppc_cfg->batching_factor;
    uint32_t witness_size = lppc_cfg->witness_size;

    vec_t round_keys = malloc_vec(nb_rounds*state_size);
    get_round_keys(round_keys);

    poly_t* poly_cst = malloc_poly_array(state_size, wit_poly_degree);
    for(uint32_t i=0; i<state_size; i++)
        poly_set_zero(poly_cst[i], wit_poly_degree);

    vec_t v = malloc_vec(packing_factor);

    for(uint32_t j=0; j<batching_factor; j++) {
        for(uint32_t i=0; i<state_size; i++) {
            for(uint32_t k=0; k<packing_factor; k++) {
                uint32_t num_round = k*batching_factor + j;
                if(num_round < nb_rounds)
                    felt_set(&v[k], &round_keys[num_round*state_size+i]);
                else
                    felt_set(&v[k], &round_keys[0+i]);
            }
            poly_interpolate(poly_cst[i], v, packing_points, packing_factor);
        }
        compute_round_verification_function_polys(&wit_polys[j*state_size], &wit_polys[(j+1)*state_size], &wit_polys[(batching_factor+1)*state_size+j*witness_size], poly_cst, &in_ppol[j*(state_size+witness_size)], state_size, wit_poly_degree);
    }

    free(round_keys);
    free(poly_cst);
    free(v);
}

void rperm_lppc_get_constraint_lin_polynomials(const reg_perm_lppc_t* lppc, const poly_t* wit_polys, const uint8_t* preprocessing_material, poly_t* in_plin, uint32_t wit_poly_degree) {
    vec_t packing_points = (vec_t) preprocessing_material;

    const reg_perm_lppc_cfg_t* lppc_cfg = rperm_lppc_get_config(lppc);
    uint32_t packing_factor = lppc_cfg->main.packing_factor;
    uint32_t state_size = lppc_cfg->state_size;
    uint32_t nb_rounds = lppc_cfg->nb_rounds;
    uint32_t batching_factor = lppc_cfg->batching_factor;
    uint32_t iv_size = lppc_cfg->iv_size;
    uint32_t y_size = lppc_cfg->y_size;
    uint32_t out_plin_degree = wit_poly_degree + (packing_factor-1);
    uint32_t offset = packing_factor*batching_factor - nb_rounds;

    poly_t* lag = malloc_poly_array(packing_factor, packing_factor-1);
    vec_t evec = malloc_vec(packing_factor);
    for(uint32_t j=0; j<packing_factor; j++) {
        vec_set_zero(evec, packing_factor);
        felt_set_one(&evec[j]);
        poly_interpolate(lag[j], evec, packing_points, packing_factor);
    }
    free(evec);

    poly_t tmp1 = malloc_poly(out_plin_degree);
    poly_t tmp2 = malloc_poly(out_plin_degree);
    for(uint32_t j=0; j<packing_factor-1; j++) {
        poly_t vani1 = lag[j];
        poly_t vani2 = lag[j+1];
        for(uint32_t i=0; i<state_size; i++) {
            poly_mul(tmp1, vani1, wit_polys[batching_factor*state_size+i], packing_factor-1, wit_poly_degree);
            poly_mul(tmp2, vani2, wit_polys[i], packing_factor-1, wit_poly_degree);
            poly_sub(in_plin[j*state_size+i], tmp1, tmp2, out_plin_degree);
        }
    }
    free(tmp1);
    free(tmp2);

    for(uint32_t j=0; j<iv_size; j++) {
        poly_t vani = lag[0];
        poly_mul(in_plin[(packing_factor-1)*state_size+j], vani, wit_polys[j], packing_factor-1, wit_poly_degree);
    }

    for(uint32_t j=0; j<y_size; j++) {
        poly_t vani = lag[packing_factor-1];
        poly_mul(in_plin[(packing_factor-1)*state_size+iv_size+j], vani, wit_polys[(batching_factor-offset)*state_size+j], packing_factor-1, wit_poly_degree);
    }

    free(lag);
}

void rperm_lppc_get_constraint_lin_polynomials_batched(const reg_perm_lppc_t* lppc, const poly_t* wit_polys, const uint8_t* preprocessing_material, const vec_t* gammas, poly_t* in_plin, uint32_t wit_poly_degree, uint32_t rho) {
    const reg_perm_lppc_cfg_t* lppc_cfg = rperm_lppc_get_config(lppc);
    uint32_t packing_factor = lppc_cfg->main.packing_factor;
    uint32_t nb_linear_constraints = lppc_cfg->main.nb_linear_constraints;
    uint32_t out_plin_degree = wit_poly_degree + (packing_factor-1);

    poly_t* in_plin2 = malloc_poly_array(nb_linear_constraints, out_plin_degree);
    rperm_lppc_get_constraint_lin_polynomials(lppc, wit_polys, preprocessing_material, in_plin2, wit_poly_degree);
    poly_t tmp_plin = malloc_poly(out_plin_degree);
    for(uint32_t num_rep=0; num_rep<rho; num_rep++) {
        poly_set_zero(in_plin[num_rep], out_plin_degree);
        for(uint32_t num=0; num<nb_linear_constraints; num++) {
            poly_mul_scalar(tmp_plin, in_plin2[num], &gammas[num_rep][num], out_plin_degree);
            poly_add(in_plin[num_rep], in_plin[num_rep], tmp_plin, out_plin_degree);
        }
    }
    free(tmp_plin);
    free(in_plin2);
}


void rperm_lppc_get_linear_result(const reg_perm_lppc_t* lppc, vec_t vt) {
    const reg_perm_lppc_cfg_t* lppc_cfg = rperm_lppc_get_config(lppc);
    uint32_t packing_factor = lppc_cfg->main.packing_factor;
    uint32_t state_size = lppc_cfg->state_size;
    uint32_t iv_size = lppc_cfg->iv_size;
    uint32_t y_size = lppc_cfg->y_size;
    vec_t iv = lppc->iv;
    vec_t y = lppc->y;

    vec_set_zero(vt, state_size*(packing_factor-1));
    vec_set(&vt[state_size*(packing_factor-1)], iv, iv_size);
    vec_set(&vt[state_size*(packing_factor-1)+iv_size], y, y_size);
}

void rperm_lppc_get_constraint_pol_evals(const reg_perm_lppc_t* lppc, const vec_t eval_points, const vec_t* evals, const uint8_t* preprocessing_material, uint32_t nb_evals, vec_t* in_epol) {
    vec_t packing_points = (vec_t) preprocessing_material;

    const reg_perm_lppc_cfg_t* lppc_cfg = rperm_lppc_get_config(lppc);
    uint32_t packing_factor = lppc_cfg->main.packing_factor;
    uint32_t batching_factor = lppc_cfg->batching_factor;
    uint32_t state_size = lppc_cfg->state_size;
    uint32_t nb_rounds = lppc_cfg->nb_rounds;
    uint32_t witness_size = lppc_cfg->witness_size;

    vec_t round_keys = malloc_vec(nb_rounds*state_size);
    get_round_keys(round_keys);

    poly_t poly_cst = malloc_poly(packing_factor-1);
    vec_t cst = malloc_vec(state_size);

    vec_t v = malloc_vec(packing_factor);

    for(uint32_t num=0; num<nb_evals; num++) {
        for(uint32_t j=0; j<batching_factor; j++) {
            for(uint32_t i=0; i<state_size; i++) {
                for(uint32_t k=0; k<packing_factor; k++) {
                    uint32_t num_round = k*batching_factor + j;
                    if(num_round < nb_rounds)
                        felt_set(&v[k], &round_keys[num_round*state_size+i]);
                    else
                        felt_set(&v[k], &round_keys[0+i]);
                }
                poly_interpolate(poly_cst, v, packing_points, packing_factor);
                poly_eval(&cst[i], poly_cst, &eval_points[num], packing_factor-1);
            }
            compute_round_verification_function(&evals[num][j*state_size], &evals[num][(j+1)*state_size], &evals[num][(batching_factor+1)*state_size+j*witness_size], cst, &in_epol[num][j*(state_size+witness_size)], state_size);
        }
    }

    free(round_keys);
    free(poly_cst);
    free(cst);
    free(v);
}

void rperm_lppc_get_constraint_lin_evals(const reg_perm_lppc_t* lppc, const vec_t eval_points, const vec_t* evals, const uint8_t* preprocessing_material, uint32_t nb_evals, vec_t* in_elin) {
    vec_t packing_points = (vec_t) preprocessing_material;

    const reg_perm_lppc_cfg_t* lppc_cfg = rperm_lppc_get_config(lppc);
    uint32_t packing_factor = lppc_cfg->main.packing_factor;
    uint32_t state_size = lppc_cfg->state_size;
    uint32_t nb_rounds = lppc_cfg->nb_rounds;
    uint32_t batching_factor = lppc_cfg->batching_factor;
    uint32_t iv_size = lppc_cfg->iv_size;
    uint32_t y_size = lppc_cfg->y_size;
    uint32_t offset = packing_factor*batching_factor - nb_rounds;

    poly_t* lag = malloc_poly_array(packing_factor, packing_factor-1);
    vec_t evec = malloc_vec(packing_factor);
    for(uint32_t j=0; j<packing_factor; j++) {
        vec_set_zero(evec, packing_factor);
        felt_set_one(&evec[j]);
        poly_interpolate(lag[j], evec, packing_points, packing_factor);
    }
    free(evec);

    vec_t lag_evals = malloc_vec(packing_factor);
    felt_t tmp1, tmp2;
    for(uint32_t num=0; num<nb_evals; num++) {
        for(uint32_t j=0; j<packing_factor; j++)
            poly_eval(&lag_evals[j], lag[j], &eval_points[num], packing_factor-1);

        for(uint32_t j=0; j<packing_factor-1; j++) {
            felt_t* vani1_eval = &lag_evals[j];
            felt_t* vani2_eval = &lag_evals[j+1];
            for(uint32_t i=0; i<state_size; i++) {
                felt_mul(&tmp1, vani1_eval, &evals[num][batching_factor*state_size+i]);
                felt_mul(&tmp2, vani2_eval, &evals[num][i]);
                felt_sub(&in_elin[num][j*state_size+i], &tmp1, &tmp2);
            }
        }

        for(uint32_t j=0; j<iv_size; j++) {
            felt_t* vani = &lag_evals[0];
            felt_mul(&in_elin[num][(packing_factor-1)*state_size+j], vani, &evals[num][j]);
        }
    
        for(uint32_t j=0; j<y_size; j++) {
            felt_t* vani = &lag_evals[packing_factor-1];
            felt_mul(&in_elin[num][(packing_factor-1)*state_size+iv_size+j], vani, &evals[num][(batching_factor-offset)*state_size+j]);
        }
    }

    free(lag);
    free(lag_evals);
}
