#ifndef __ANEMOI_ROUND_H__
#define __ANEMOI_ROUND_H__

#include "field.h"

void felt_mul_by_generator(felt_t* c, const felt_t* a);
void get_permutation_parameters(uint32_t* state_size, uint32_t* nb_rounds, uint32_t* round_wit_size, uint32_t* verification_degree);
void get_round_keys(vec_t round_keys);
void apply_sbox_layer(const vec_t in_state, vec_t out_state, uint32_t state_size);
void get_sbox_parameters(uint8_t* alpha, felt_t* beta, felt_t* delta);



#endif /* __ANEMOI_ROUND_H__ */
