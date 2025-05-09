#include <stdint.h>
#include "lppc-regular-perm.h"
#include "smallwood.h"

typedef struct {
    // LPPC Parameters
    uint32_t batching_factor;
    uint32_t iv_size;
    uint32_t y_size;
    // SmallWood Parameters
    uint32_t tree_height1;
    uint32_t tree_arity1;
    uint32_t tree_height2;
    uint32_t tree_arity2;
    uint32_t tree_nb_leaves;
    uint32_t rho;
    uint32_t piop_nb_opened_evals;
    uint32_t beta;
    uint32_t decs_nb_opened_evals;
    uint32_t decs_eta;
    uint32_t decs_pow_bits;
} anemoi_parameters_t;

int get_config_lppc_smallwood(uint32_t variant, reg_perm_lppc_cfg_t** lppc_cfg, smallwood_t** sw) {
    if(lppc_cfg == NULL)
        return 0;

    anemoi_parameters_t params = {0};
    if(variant == 0) {
        params.batching_factor = 7;
        params.iv_size = 1;
        params.y_size = 1;

        params.tree_height1 = 14;
        params.tree_arity1 = 2;
        params.tree_height2 = 0;
        params.tree_arity2 = 0;
        params.tree_nb_leaves = 4096*4;
        params.rho = 1;
        params.piop_nb_opened_evals = 1;
        params.beta = 1;
        params.decs_nb_opened_evals = 13;
        params.decs_eta = 2;
        params.decs_pow_bits = 8;

    } else if(variant == 1) {
        params.batching_factor = 7;
        params.iv_size = 1;
        params.y_size = 1;

        params.tree_height1 = 6;
        params.tree_arity1 = 4;
        params.tree_height2 = 0;
        params.tree_arity2 = 0;
        params.tree_nb_leaves = 4096;
        params.rho = 1;
        params.piop_nb_opened_evals = 1;
        params.beta = 1;
        params.decs_nb_opened_evals = 17;
        params.decs_eta = 2;
        params.decs_pow_bits = 7;
        
    } else if(variant == 2) {
        params.batching_factor = 7;
        params.iv_size = 1;
        params.y_size = 1;

        params.tree_height1 = 5;
        params.tree_arity1 = 4;
        params.tree_height2 = 0;
        params.tree_arity2 = 0;
        params.tree_nb_leaves = 1024;
        params.rho = 1;
        params.piop_nb_opened_evals = 1;
        params.beta = 1;
        params.decs_nb_opened_evals = 24;
        params.decs_eta = 2;
        params.decs_pow_bits = 8;
        
    } else {
        printf("unknown signature instance\n");
        return -1;
    }
    *lppc_cfg = rperm_lppc_malloc_config(params.batching_factor, params.iv_size, params.y_size);
    if(*lppc_cfg == NULL)
        return -1;

    if(sw != NULL) {
        uint32_t height = params.tree_height1 + params.tree_height2;
        uint32_t* arities = malloc(height*sizeof(height));
        if(arities == NULL) {
            free(*lppc_cfg);
            return -1;
        }
        for(uint32_t i=0; i<params.tree_height1; i++)
            arities[i] = params.tree_arity1;
        for(uint32_t i=0; i<params.tree_height2; i++)
            arities[params.tree_height1+i] = params.tree_arity2;
        merkle_tree_cfg_t tree_cfg = {
            .nb_leaves = params.tree_nb_leaves,
            .height = height,
            .arities = arities
        };
        smallwood_cfg_t sw_cfg = {
            .rho = params.rho,
            .nb_opened_evals = params.piop_nb_opened_evals,
            .beta = params.beta,
            .piop_format_challenge = 0,
            .decs_nb_evals = params.tree_nb_leaves,
            .decs_nb_opened_evals = params.decs_nb_opened_evals,
            .decs_eta = params.decs_eta,
            .decs_pow_bits = params.decs_pow_bits,
            .decs_use_commitment_tapes = 0,
            .decs_format_challenge = 0,
            .decs_tree_cfg = &tree_cfg,
        };
        *sw = malloc_smallwood(rperm_lppc_get_config_lppc(*lppc_cfg), &sw_cfg);
        free(arities);
        if(*sw == NULL) {
            free(*lppc_cfg);
            return -1;
        }
    }

    return 0;
}
