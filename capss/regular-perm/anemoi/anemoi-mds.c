#include "anemoi-mds.h"
#include "anemoi-round.h"

static void poly_mul_by_generator(poly_t c, const poly_t a, uint32_t degree) {
    for(uint32_t num=0; num<=degree; num++)
        felt_mul_by_generator(&c[num], &a[num]);
}

static void felt_swap(felt_t* a, felt_t* b) {
    felt_t c;
    felt_set(&c, a);
    felt_set(a, b);
    felt_set(b, &c);
}

static void poly_swap(poly_t a, poly_t b, uint32_t degree) {
    felt_t tmp;
    for(uint32_t num=0; num<=degree; num++) {
        felt_set(&tmp, &a[num]);
        felt_set(&a[num], &b[num]);
        felt_set(&b[num], &tmp);
    }
}

static void apply_mds_internal(vec_t state, uint32_t nb_cols) {
    felt_t tmp;

    if(nb_cols == 3) {
        // let tmp = state[0] + Self::mul_by_generator(&state[2]);
        felt_mul_by_generator(&tmp, &state[2]);
        felt_add(&tmp, &state[0], &tmp);

        // state[2] += state[1];
        felt_add(&state[2], &state[2], &state[1]);
        // state[2] += Self::mul_by_generator(&state[0]);

        // state[0] = tmp + state[2];
        felt_add(&state[0], &tmp, &state[2]);
        // state[1] += tmp;
        felt_add(&state[1], &state[1], &tmp);

    } else if(nb_cols == 4) {
        // state[0] += state[1];
        felt_add(&state[0], &state[0], &state[1]);
        // state[2] += state[3];
        felt_add(&state[2], &state[2], &state[3]);
        // state[3] += Self::mul_by_generator(&state[0]);
        felt_mul_by_generator(&tmp, &state[0]);
        felt_add(&state[3], &state[3], &tmp);
        // state[1] = Self::mul_by_generator(&(state[1] + state[2]));
        felt_add(&tmp, &state[1], &state[2]);
        felt_mul_by_generator(&state[1], &tmp);

        // state[0] += state[1];
        felt_add(&state[0], &state[0], &state[1]);
        // state[2] += Self::mul_by_generator(&state[3]);
        felt_mul_by_generator(&tmp, &state[3]);
        felt_add(&state[2], &state[2], &tmp);
        // state[1] += state[2];
        felt_add(&state[1], &state[1], &state[2]);
        // state[3] += state[0];
        felt_add(&state[3], &state[3], &state[0]);
    }
}

static void apply_mds_internal_poly(poly_t* state, uint32_t nb_cols, uint32_t degree) {
    poly_t tmp = malloc_poly(degree);

    if(nb_cols == 3) {
        // let tmp = state[0] + Self::mul_by_generator(&state[2]);
        poly_mul_by_generator(tmp, state[2], degree);
        poly_add(tmp, state[0], tmp, degree);

        // state[2] += state[1];
        poly_add(state[2], state[2], state[1], degree);
        // state[2] += Self::mul_by_generator(&state[0]);

        // state[0] = tmp + state[2];
        poly_add(state[0], tmp, state[2], degree);
        // state[1] += tmp;
        poly_add(state[1], state[1], tmp, degree);

    } else if(nb_cols == 4) {
        // state[0] += state[1];
        poly_add(state[0], state[0], state[1], degree);
        // state[2] += state[3];
        poly_add(state[2], state[2], state[3], degree);
        // state[3] += Self::mul_by_generator(&state[0]);
        poly_mul_by_generator(tmp, state[0], degree);
        poly_add(state[3], state[3], tmp, degree);
        // state[1] = Self::mul_by_generator(&(state[1] + state[2]));
        poly_add(tmp, state[1], state[2], degree);
        poly_mul_by_generator(state[1], tmp, degree);

        // state[0] += state[1];
        poly_add(state[0], state[0], state[1], degree);
        // state[2] += Self::mul_by_generator(&state[3]);
        poly_mul_by_generator(tmp, state[3], degree);
        poly_add(state[2], state[2], tmp, degree);
        // state[1] += state[2];
        poly_add(state[1], state[1], state[2], degree);
        // state[3] += state[0];
        poly_add(state[3], state[3], state[0], degree);
    }

    free(tmp);
}

void apply_mds_layer(vec_t state, uint32_t state_size) {
    uint32_t nb_cols = state_size / 2;

    if(nb_cols == 1) {
        // state[1] += state[0];
        felt_add(&state[1], &state[1], &state[0]);
        // state[0] += state[1];
        felt_add(&state[0], &state[0], &state[1]);    

    } else if(nb_cols == 2) {
        felt_t tmp;

        // state[0] += Self::mul_by_generator(&state[1]);
        felt_mul_by_generator(&tmp, &state[1]);
        felt_add(&state[0], &state[0], &tmp);

        // state[1] += Self::mul_by_generator(&state[0]);
        felt_mul_by_generator(&tmp, &state[0]);
        felt_add(&state[1], &state[1], &tmp);

        // state[3] += Self::mul_by_generator(&state[2]);
        felt_mul_by_generator(&tmp, &state[2]);
        felt_add(&state[3], &state[3], &tmp);

        // state[2] += Self::mul_by_generator(&state[3]);
        felt_mul_by_generator(&tmp, &state[3]);
        felt_add(&state[2], &state[2], &tmp);

        // state.swap(2, 3);
        felt_swap(&state[2], &state[3]);

        // state[2] += state[0];
        felt_add(&state[2], &state[2], &state[0]);
        // state[3] += state[1];
        felt_add(&state[3], &state[3], &state[1]);    

        // state[0] += state[2];
        felt_add(&state[0], &state[0], &state[2]);
        // state[1] += state[3];
        felt_add(&state[1], &state[1], &state[3]);    

    } else if(nb_cols == 3) {
        felt_t tmp;

        // Self::mds_internal(&mut state[..Self::NUM_COLUMNS]);
        apply_mds_internal(&state[0], nb_cols);
        // state[Self::NUM_COLUMNS..].rotate_left(1);
        felt_set(&tmp, &state[nb_cols+0]);
        felt_set(&state[nb_cols+0], &state[nb_cols+1]);
        felt_set(&state[nb_cols+1], &state[nb_cols+2]);
        felt_set(&state[nb_cols+2], &tmp);
        // Self::mds_internal(&mut state[Self::NUM_COLUMNS..]);
        apply_mds_internal(&state[nb_cols], nb_cols);

        // PHT layer
        // state[3] += state[0];
        felt_add(&state[3], &state[3], &state[0]);
        // state[4] += state[1];
        felt_add(&state[4], &state[4], &state[1]);
        // state[5] += state[2];
        felt_add(&state[5], &state[5], &state[2]);

        // state[0] += state[3];
        felt_add(&state[0], &state[0], &state[3]);
        // state[1] += state[4];
        felt_add(&state[1], &state[1], &state[4]);
        // state[2] += state[5];
        felt_add(&state[2], &state[2], &state[5]);

    } else if(nb_cols == 4) {
        felt_t tmp;

        // Self::mds_internal(&mut state[..Self::NUM_COLUMNS]);
        apply_mds_internal(&state[0], nb_cols);
        // state[Self::NUM_COLUMNS..].rotate_left(1);
        felt_set(&tmp, &state[nb_cols+0]);
        felt_set(&state[nb_cols+0], &state[nb_cols+1]);
        felt_set(&state[nb_cols+1], &state[nb_cols+2]);
        felt_set(&state[nb_cols+2], &state[nb_cols+3]);
        felt_set(&state[nb_cols+3], &tmp);
        // Self::mds_internal(&mut state[Self::NUM_COLUMNS..]);
        apply_mds_internal(&state[nb_cols], nb_cols);

        // state[4] += state[0];
        felt_add(&state[4], &state[4], &state[0]);
        // state[5] += state[1];
        felt_add(&state[5], &state[5], &state[1]);
        // state[6] += state[2];
        felt_add(&state[6], &state[6], &state[2]);
        // state[7] += state[3];
        felt_add(&state[7], &state[7], &state[3]);

        // state[0] += state[4];
        felt_add(&state[0], &state[0], &state[4]);
        // state[1] += state[5];
        felt_add(&state[1], &state[1], &state[5]);
        // state[2] += state[6];
        felt_add(&state[2], &state[2], &state[6]);
        // state[3] += state[7];
        felt_add(&state[3], &state[3], &state[7]);
    }
}

void apply_mds_layer_poly(poly_t* state, uint32_t state_size, uint32_t degree) {
    uint32_t nb_cols = state_size / 2;
    poly_t tmp = malloc_poly(degree);

    if(nb_cols == 1) {

        // state[1] += state[0];
        poly_add(state[1], state[1], state[0], degree);
        // state[0] += state[1];
        poly_add(state[0], state[0], state[1], degree);    

    } else if(nb_cols == 2) {

        // state[0] += Self::mul_by_generator(&state[1]);
        poly_mul_by_generator(tmp, state[1], degree);
        poly_add(state[0], state[0], tmp, degree);

        // state[1] += Self::mul_by_generator(&state[0]);
        poly_mul_by_generator(tmp, state[0], degree);
        poly_add(state[1], state[1], tmp, degree);

        // state[3] += Self::mul_by_generator(&state[2]);
        poly_mul_by_generator(tmp, state[2], degree);
        poly_add(state[3], state[3], tmp, degree);

        // state[2] += Self::mul_by_generator(&state[3]);
        poly_mul_by_generator(tmp, state[3], degree);
        poly_add(state[2], state[2], tmp, degree);

        // state.swap(2, 3);
        poly_swap(state[2], state[3], degree);

        // state[2] += state[0];
        poly_add(state[2], state[2], state[0], degree);
        // state[3] += state[1];
        poly_add(state[3], state[3], state[1], degree);    

        // state[0] += state[2];
        poly_add(state[0], state[0], state[2], degree);
        // state[1] += state[3];
        poly_add(state[1], state[1], state[3], degree);    

    } else if(nb_cols == 3) {

        // Self::mds_internal(&mut state[..Self::NUM_COLUMNS]);
        apply_mds_internal_poly(&state[0], nb_cols, degree);
        // state[Self::NUM_COLUMNS..].rotate_left(1);
        poly_set(tmp, state[nb_cols+0], degree);
        poly_set(state[nb_cols+0], state[nb_cols+1], degree);
        poly_set(state[nb_cols+1], state[nb_cols+2], degree);
        poly_set(state[nb_cols+2], tmp, degree);
        // Self::mds_internal(&mut state[Self::NUM_COLUMNS..]);
        apply_mds_internal_poly(&state[nb_cols], nb_cols, degree);

        // PHT layer
        // state[3] += state[0];
        poly_add(state[3], state[3], state[0], degree);
        // state[4] += state[1];
        poly_add(state[4], state[4], state[1], degree);
        // state[5] += state[2];
        poly_add(state[5], state[5], state[2], degree);

        // state[0] += state[3];
        poly_add(state[0], state[0], state[3], degree);
        // state[1] += state[4];
        poly_add(state[1], state[1], state[4], degree);
        // state[2] += state[5];
        poly_add(state[2], state[2], state[5], degree);

    } else if(nb_cols == 4) {

        // Self::mds_internal(&mut state[..Self::NUM_COLUMNS]);
        apply_mds_internal_poly(&state[0], nb_cols, degree);
        // state[Self::NUM_COLUMNS..].rotate_left(1);
        poly_set(tmp, state[nb_cols+0], degree);
        poly_set(state[nb_cols+0], state[nb_cols+1], degree);
        poly_set(state[nb_cols+1], state[nb_cols+2], degree);
        poly_set(state[nb_cols+2], state[nb_cols+3], degree);
        poly_set(state[nb_cols+3], tmp, degree);
        // Self::mds_internal(&mut state[Self::NUM_COLUMNS..]);
        apply_mds_internal_poly(&state[nb_cols], nb_cols, degree);

        // state[4] += state[0];
        poly_add(state[4], state[4], state[0], degree);
        // state[5] += state[1];
        poly_add(state[5], state[5], state[1], degree);
        // state[6] += state[2];
        poly_add(state[6], state[6], state[2], degree);
        // state[7] += state[3];
        poly_add(state[7], state[7], state[3], degree);

        // state[0] += state[4];
        poly_add(state[0], state[0], state[4], degree);
        // state[1] += state[5];
        poly_add(state[1], state[1], state[5], degree);
        // state[2] += state[6];
        poly_add(state[2], state[2], state[6], degree);
        // state[3] += state[7];
        poly_add(state[3], state[3], state[7], degree);
    }

    free(tmp);
}
