#include "permutation-round.h"
#include "anemoi-mds.h"
#include "anemoi-round.h"

void compute_round_function_with_witness(const vec_t in_state, const vec_t cst, uint32_t state_size, vec_t out_state, vec_t round_wit) {
    vec_t state = malloc_vec(state_size);
    (void) round_wit; // no round witness

    // ARK Layer (Add Round Keys)
    vec_add(state, in_state, cst, state_size);

    // MDS Layer
    apply_mds_layer(state, state_size);

    // SBox Layer
    apply_sbox_layer(state, out_state, state_size);
    free(state);
}

void compute_round_verification_function(const vec_t in_state, const vec_t out_state, const vec_t round_wit, const vec_t cst, vec_t out, uint32_t state_size) {
    felt_t beta, delta;
    uint8_t alpha;
    get_sbox_parameters(&alpha, &beta, &delta);
    (void) round_wit; // no round witness
    
    // ARK Layer (Add Round Keys)
    vec_t state = malloc_vec(state_size);
    vec_add(state, in_state, cst, state_size);

    // MDS Layer
    apply_mds_layer(state, state_size);

    // SBox Layer
    uint32_t nb_cols = state_size / 2;

    // Compute t = x - (beta * y**2)
    for(uint32_t i=0; i<nb_cols; i++) {
        felt_t* x = &state[i];
        felt_t* y = &state[nb_cols+i];
        felt_t* u = &out_state[i];
        felt_t* v = &out_state[nb_cols+i];
        felt_t* out1 = &out[i];
        felt_t* out2 = &out[nb_cols+i];

        felt_t t;
        felt_mul(&t, y, y);
        felt_mul(&t, &t, &beta);
        felt_sub(&t, x, &t);
    
        felt_t y_minus_v;
        felt_sub(&y_minus_v, y, v);
        if(alpha == 3) {
            felt_mul(out1, &y_minus_v, &y_minus_v);
            felt_mul(out1, out1, &y_minus_v);
        } else if(alpha == 5) {
            felt_mul(out1, &y_minus_v, &y_minus_v);
            felt_mul(out1, out1, out1);
            felt_mul(out1, out1, &y_minus_v);
        } else if(alpha == 7) {
            felt_mul(out1, &y_minus_v, &y_minus_v);
            felt_mul(out1, out1, &y_minus_v);
            felt_mul(out1, out1, out1);
            felt_mul(out1, out1, &y_minus_v);
        } else {
            printf("Error: unknown alpha: %d\n", alpha);
        }
        felt_sub(out1, out1, &t);
    
        felt_t tmp2;
        felt_mul(&tmp2, v, v);
        felt_mul(&tmp2, &tmp2, &beta);
        felt_add(&tmp2, &tmp2, &delta);
        felt_sub(out2, u, &tmp2);
        felt_sub(out2, out2, &t);
    }

    free(state);
}

void compute_round_verification_function_polys(const poly_t* in_state, const poly_t* out_state, const poly_t* round_wit, const poly_t* cst, poly_t* out, uint32_t state_size, uint32_t wit_poly_degree) {
    felt_t beta, delta;
    uint8_t alpha;
    get_sbox_parameters(&alpha, &beta, &delta);
    (void) round_wit; // no round witness


    // ARK Layer (Add Round Keys)
    poly_t* state = malloc_poly_array(state_size, wit_poly_degree);
    for(uint32_t i=0; i<state_size; i++)
        poly_add(state[i], in_state[i], cst[i], wit_poly_degree);

    // MDS Layer
    apply_mds_layer_poly(state, state_size, wit_poly_degree);

    // SBox Layer
    uint32_t nb_cols = state_size / 2;

    // Compute t = x - (beta * y**2)
    poly_t t = malloc_poly(2*wit_poly_degree);
    poly_t y_minus_v = malloc_poly(wit_poly_degree);
    poly_t tmp2 = malloc_poly(2*wit_poly_degree);
    for(uint32_t i=0; i<nb_cols; i++) {
        poly_t x = state[i];
        poly_t y = state[nb_cols+i];
        poly_t u = out_state[i];
        poly_t v = out_state[nb_cols+i];
        poly_t out1 = out[i];
        poly_t out2 = out[nb_cols+i];

        poly_mul(t, y, y, wit_poly_degree, wit_poly_degree);
        poly_mul_scalar(t, t, &beta, 2*wit_poly_degree);
        poly_neg(t, t, 2*wit_poly_degree);
        poly_add(t, t, x, wit_poly_degree);

        poly_sub(y_minus_v, y, v, wit_poly_degree);
        if(alpha == 3) {
            poly_mul(out1, y_minus_v, y_minus_v, wit_poly_degree, wit_poly_degree);
            poly_mul(out1, out1, y_minus_v, 2*wit_poly_degree, wit_poly_degree);
        } else if(alpha == 5) {
            poly_mul(out1, y_minus_v, y_minus_v, wit_poly_degree, wit_poly_degree);
            poly_mul(out1, out1, out1, 2*wit_poly_degree, 2*wit_poly_degree);
            poly_mul(out1, out1, y_minus_v, 4*wit_poly_degree, wit_poly_degree);
        } else if(alpha == 7) {
            poly_mul(out1, y_minus_v, y_minus_v, wit_poly_degree, wit_poly_degree);
            poly_mul(out1, out1, y_minus_v, 2*wit_poly_degree, wit_poly_degree);
            poly_mul(out1, out1, out1, 3*wit_poly_degree, 3*wit_poly_degree);
            poly_mul(out1, out1, y_minus_v, 6*wit_poly_degree, wit_poly_degree);
        } else {
            printf("Error: unknown alpha: %d\n", alpha);
        }
        poly_sub(out1, out1, t, 2*wit_poly_degree);

        poly_mul(tmp2, v, v, wit_poly_degree, wit_poly_degree);
        poly_mul_scalar(tmp2, tmp2, &beta, 2*wit_poly_degree);
        felt_add(&tmp2[0], &tmp2[0], &delta);
        poly_neg(tmp2, tmp2, 2*wit_poly_degree);
        poly_add(tmp2, tmp2, u, wit_poly_degree);
        poly_sub(tmp2, tmp2, t, 2*wit_poly_degree);
        poly_set_zero(out2, alpha*wit_poly_degree);
        poly_set(out2, tmp2, 2*wit_poly_degree);
    }
    free(y_minus_v);
    free(tmp2);
    free(t);

    free(state);
}
