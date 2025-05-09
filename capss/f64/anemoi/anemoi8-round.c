
#include "permutation-round.h"

extern void f64_anemoi_8_get_nb_rounds(uint32_t* nb_rounds);
extern void f64_anemoi_8_get_sbox_parameters(uint8_t* alpha, uint64_t* beta, uint64_t* delta);
extern void f64_anemoi_8_get_round_keys(uint64_t* round_keys);
extern void f64_anemoi_8_sbox_layer(uint64_t* out, const uint64_t* in);

void felt_mul_by_generator(felt_t* c, const felt_t* a) {
    felt_t gen;
    felt_from_uint32(&gen, 7);
    felt_mul(c, a, &gen);
}

void get_permutation_parameters(uint32_t* state_size, uint32_t* nb_rounds, uint32_t* round_wit_size, uint32_t* verification_degree) {
    *state_size = 8;
    *round_wit_size = 0;
    f64_anemoi_8_get_nb_rounds(nb_rounds);
    uint8_t alpha;
    f64_anemoi_8_get_sbox_parameters(&alpha, NULL, NULL);
    *verification_degree = alpha;
}

void get_round_keys(vec_t round_keys) {
    f64_anemoi_8_get_round_keys(round_keys);
}

void apply_sbox_layer(const vec_t in_state, vec_t out_state, uint32_t state_size) {
    (void) state_size;
    f64_anemoi_8_sbox_layer(out_state, in_state);
}

void get_sbox_parameters(uint8_t* alpha, felt_t* beta, felt_t* delta) {
    f64_anemoi_8_get_sbox_parameters(alpha, beta, delta);
}
