#include "lppc.h"
#include "lppc-regular-perm.h"
#include "smallwood.h"
#include <stdio.h>

#define BATCHING_FACTOR 7
#define STATE_SIZE 2
#define Y_SIZE 1
#define IV_SIZE 1

int malloc_example_lppc_with_witness(lppc_t** lppc, vec_t* witness) {
    int ret = 0;
    *lppc = NULL;
    *witness = NULL;

    reg_perm_lppc_t** lppc_local = (reg_perm_lppc_t**) lppc;
    reg_perm_lppc_cfg_t* lppc_cfg = rperm_lppc_malloc_config(BATCHING_FACTOR, IV_SIZE, Y_SIZE);
    if(lppc_cfg == NULL) {
        return -1;
    }
    vec_t secret;
    ret = rperm_lppc_random(lppc_cfg, lppc_local, &secret);
    if(ret != 0) {
        free(lppc_cfg);
        return -2;
    }
    *witness = malloc_vec(lppc_get_witness_size(rperm_lppc_get_config_lppc(lppc_cfg)));
    free(lppc_cfg);
    if(*witness == NULL) {
        free(*lppc_local);
        free(secret);
        return -3;
    }
    ret = rperm_lppc_build_witness(*lppc_local, secret, *witness); 
    free(secret);
    if(ret != 0) {
        free(*lppc_local);
        free(*witness);
        return -1;
    }
    return 0;
}

#define PIOP_RHO 1
#define PIOP_NB_OPENED_EVALS 1
#define PCS_BETA 1

//#define DECS_NB_EVALS 4096
//#define DECS_NB_OPENED_EVALS 17
#define DECS_NB_EVALS (4096*4)
#define DECS_NB_OPENED_EVALS 13
#define DECS_ETA 2
#define DECS_POW_BITS 8

#if 0
#define TREE_HEIGHT 14
#define TREE_ARITIES {2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}
#else
#define TREE_HEIGHT 7
#define TREE_ARITIES {4, 4, 4, 4, 4, 4, 4}
#endif

smallwood_t* malloc_example_smallwood(const lppc_t* lppc) {
    uint32_t arities[TREE_HEIGHT] = TREE_ARITIES;
    merkle_tree_cfg_t tree_cfg = {
        .nb_leaves = DECS_NB_EVALS,
        .height = TREE_HEIGHT,
        .arities = arities
    };
    smallwood_cfg_t sw_cfg = {
        .rho = PIOP_RHO,
        .nb_opened_evals = PIOP_NB_OPENED_EVALS,
        .beta = PCS_BETA,
        .decs_nb_evals = DECS_NB_EVALS,
        .decs_nb_opened_evals = DECS_NB_OPENED_EVALS,
        .decs_eta = DECS_ETA,
        .decs_pow_bits = DECS_POW_BITS,
        .decs_use_commitment_tapes = 0,
        .decs_tree_cfg = &tree_cfg,
    };
    return malloc_smallwood(lppc_get_config(lppc), &sw_cfg);
}
