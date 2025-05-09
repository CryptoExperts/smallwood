#ifndef __PERMUTATION_ROUND_H__
#define __PERMUTATION_ROUND_H__

#include "field.h"

void get_permutation_parameters(uint32_t* state_size, uint32_t* nb_rounds, uint32_t* round_wit_size, uint32_t* verification_degree);
void get_round_keys(vec_t round_keys);
void compute_round_function_with_witness(const vec_t in_state, const vec_t cst, uint32_t state_size, vec_t out_state, vec_t round_wit);

void compute_round_verification_function(const vec_t in_state, const vec_t out_state, const vec_t round_wit, const vec_t cst, vec_t out, uint32_t state_size);
void compute_round_verification_function_polys(const poly_t* in_state, const poly_t* out_state, const poly_t* round_wit, const poly_t* cst, poly_t* out, uint32_t state_size, uint32_t wit_poly_degree);


#endif /* __PERMUTATION_ROUND_H__ */
