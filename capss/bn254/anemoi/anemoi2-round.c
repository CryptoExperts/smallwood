
#include "permutation-round.h"

extern void bn254_anemoi_2_get_nb_rounds(uint32_t* nb_rounds);
extern void bn254_anemoi_2_get_sbox_parameters(uint8_t* alpha, uint8_t* beta, uint8_t* delta);
extern void bn254_anemoi_2_get_round_keys(uint8_t* round_keys);
extern void bn254_anemoi_2_sbox_layer(uint8_t* out, const uint8_t* in);

void felt_mul_by_generator(felt_t* c, const felt_t* a) {
    felt_t gen;
    felt_from_uint32(&gen, 3);
    felt_mul(c, a, &gen);
}

void get_permutation_parameters(uint32_t* state_size, uint32_t* nb_rounds, uint32_t* round_wit_size, uint32_t* verification_degree) {
    *state_size = 2;
    *round_wit_size = 0;
    bn254_anemoi_2_get_nb_rounds(nb_rounds);
    uint8_t alpha;
    bn254_anemoi_2_get_sbox_parameters(&alpha, NULL, NULL);
    *verification_degree = alpha;
}

void get_round_keys(vec_t round_keys) {
    bn254_anemoi_2_get_round_keys((uint8_t*) round_keys);
}

void apply_sbox_layer(const vec_t in_state, vec_t out_state, uint32_t state_size) {
    (void) state_size;
    bn254_anemoi_2_sbox_layer((uint8_t*) out_state, (uint8_t*) in_state);
}

void get_sbox_parameters(uint8_t* alpha, felt_t* beta, felt_t* delta) {
    bn254_anemoi_2_get_sbox_parameters(alpha, *beta, *delta);
}

