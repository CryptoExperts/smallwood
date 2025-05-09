#ifndef __ANEMOI_MDS_H__
#define __ANEMOI_MDS_H__

#include "field.h"

void apply_mds_layer(vec_t state, uint32_t state_size);
void apply_mds_layer_poly(poly_t* state, uint32_t state_size, uint32_t degree);

#endif /* __ANEMOI_MDS_H__ */
