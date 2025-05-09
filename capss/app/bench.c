#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "capss-sign.h"
#include "benchmark.h"
#include "timing.h"

#define MLEN 32

#define B_KEY_GENERATION 0
#define B_SIGN_ALGO 1
#define B_VERIFY_ALGO 2
#define NUMBER_OF_ALGO_BENCHES 3

#ifdef DETAILED_BENCHMARK
btimer_t timers[NUMBER_OF_BENCHES];

#ifdef MEASURE_CYCLES
#define display_timer(label,num) printf("   - " label ": %f ms (%f cycles)\n", btimer_get(&timers[num]), btimer_get_cycles(&timers[num]))
#else
#define display_timer(label,num) printf("   - " label ": %f ms\n", btimer_get(&timers[num]))
#endif
#endif

#ifdef MEASURE_CYCLES
#define display_timer_algo(label,num) printf("   - " label ": %f ms (%f cycles)\n", btimer_get(&timers_algos[num]), btimer_get_cycles(&timers_algos[num]))
#else
#define display_timer_algo(label,num) printf("   - " label ": %f ms\n", btimer_get(&timers_algos[num]))
#endif

int randombytes(unsigned char* x, unsigned long long xlen) {
    for(unsigned long long j=0; j<xlen; j++)
        x[j] = (uint8_t) rand();
    return 0;
}

static inline int get_number_of_tests(int argc, char *argv[], int default_value) {
    int nb_tests = default_value;
    if(argc == 2) {
        if( sscanf(argv[1], "%d", &nb_tests) != 1) {
            printf("Integer awaited.\n");
            return -1;
        } else if( nb_tests <= 0 ) {
            printf("Need to positive integer.\n");
            return -1;
        }
    }
    return nb_tests;
}

int main(int argc, char *argv[]) {
    srand((unsigned int) time(NULL));
    int ret;

    int nb_tests = get_number_of_tests(argc, argv, 1);
    if(nb_tests < 0)
        exit(EXIT_FAILURE);

    #ifdef DETAILED_BENCHMARK
    for(int num=0; num<NUMBER_OF_BENCHES; num++)
        btimer_init(&timers[num]);
    #endif

    btimer_t timers_algos[NUMBER_OF_ALGO_BENCHES];
    for(int j=0; j<NUMBER_OF_ALGO_BENCHES; j++)
        btimer_init(&timers_algos[j]);

    unsigned long long pk_bytesize, sk_bytesize, sig_max_bytesize;
    ret = crypto_sign_get_sizes(&pk_bytesize, &sk_bytesize, &sig_max_bytesize);
    if(ret) {
        printf("Failure: crypto_sign_get_sizes\n");
        return 0;
    }

    uint8_t* pk = malloc(pk_bytesize);
    uint8_t* sk = malloc(sk_bytesize);
    uint8_t* sm = malloc(sig_max_bytesize+MLEN);
    unsigned long long smlen;

    uint32_t score = 0;
    uint32_t average_signature_size = 0;
    for(int num_test=0; num_test<nb_tests; num_test++) {
        #ifdef DETAILED_BENCHMARK
        for(int num=0; num<NUMBER_OF_BENCHES; num++)
            btimer_count(&timers[num]);
        #endif

        btimer_start(&timers_algos[B_KEY_GENERATION]);
        ret = crypto_sign_keypair(pk, sk);
        btimer_end(&timers_algos[B_KEY_GENERATION]);
        btimer_count(&timers_algos[B_KEY_GENERATION]);
        if(ret) {
            printf("Failure (num %d): crypto_sign_keypair\n", num_test);
            continue;
        }


        uint8_t m[MLEN] = {1, 2, 3, 4};
        uint8_t m2[MLEN] = {0};
        unsigned long long m2len = 0;

        btimer_start(&timers_algos[B_SIGN_ALGO]);
        ret = crypto_sign(sm, &smlen, m, MLEN, sk);
        btimer_end(&timers_algos[B_SIGN_ALGO]);
        btimer_count(&timers_algos[B_SIGN_ALGO]);
        if(ret) {
            printf("Failure (num %d): crypto_sign_signature\n", num_test);
            continue;
        }

        btimer_start(&timers_algos[B_VERIFY_ALGO]);
        ret = crypto_sign_open(m2, &m2len, sm, smlen, pk);
        btimer_end(&timers_algos[B_VERIFY_ALGO]);
        btimer_count(&timers_algos[B_VERIFY_ALGO]);
        if(ret) {
            printf("Failure (num %d): crypto_sign_verify\n", num_test);
            continue;
        }
        if(m2len != MLEN) {
            printf("Failure (num %d): message size does not match\n", num_test);
            continue;
        }
        if(memcmp(m, m2, MLEN) != 0) {
            printf("Failure (num %d): message does not match\n", num_test);
            continue;
        }

        average_signature_size += smlen - sizeof(uint32_t) - MLEN;
        score++;        
    }
    free(pk);
    free(sk);
    free(sm);

    printf("\n===== SUMMARY =====\n");
    printf(" - Score: %d/%d\n", score, nb_tests);
    printf(" - Sizes:\n");
    printf("   - PK: %llu B\n", pk_bytesize);
    printf("   - SK: %llu B\n", sk_bytesize);
    if(score)
        printf("   - Signature: %d B\n", average_signature_size/score);
    else
        printf("   - Signature: NaN\n");
    printf(" - Timing:\n");
    display_timer_algo("Key Generation", B_KEY_GENERATION);
    display_timer_algo("Sign", B_SIGN_ALGO);
    display_timer_algo("Verification", B_VERIFY_ALGO);

    #ifdef DETAILED_BENCHMARK
    printf("\n===== DETAILED DETAILED_BENCHMARK =====\n");
    printf(" - DECS\n");
    display_timer("[DECS.Commit] Compute Leaves", DECS_COMMIT_COMPUTE_LEAVES);
    display_timer("[DECS.Commit] Hash Leaves", DECS_COMMIT_HASH_LEAVES);
    display_timer("[DECS.Commit] Merkle Tree", DECS_COMMIT_MERKLE_TREE);
    display_timer("[DECS.Commit] XOF Merkle Root", DECS_COMMIT_XOF_MERKLE_ROOT);
    display_timer("[DECS.Commit] Compute DEC Polys", DECS_COMMIT_COMPUTE_DEC_POLYS);
    display_timer("[DECS.Open] Open Tree", DECS_OPEN_OPEN_TREE);
    display_timer("[DECS.Open] Compute Evals", DECS_OPEN_COMPUTE_EVALS);
    display_timer("[DECS.Recompute] Hash Leaves", DECS_RECOMPUTE_HASH_LEAVES);
    display_timer("[DECS.Recompute] Retrieve Root", DECS_RECOMPUTE_RETRIEVE_ROOT);
    display_timer("[DECS.Recompute] XOF Merkle Root", DECS_RECOMPUTE_XOF_MERKLE_ROOT);
    display_timer("[DECS.Recompute] Compute DEC Evals", DECS_RECOMPUTE_COMPUTE_DEC_EVALS);
    display_timer("[DECS.Recompute] Retrieve DEC Polys", DECS_RECOMPUTE_RETRIEVE_DEC_POLYS);
    printf(" - LVCS\n");
    display_timer("[LVCS.Commit] Interpolate", LVCS_COMMIT_INTERPOLATE);
    display_timer("[LVCS.Open] Compute Extended Combis", LVCS_OPEN_COMPUTE_EXTENDED_COMBIS);
    display_timer("[LVCS.Open] Compute Opening Challenge", LVCS_OPEN_COMPUTE_OPENING_CHALLENGE);
    display_timer("[LVCS.Recompute] Compute Opening Challenge", LVCS_RECOMPUTE_COMPUTE_OPENING_CHALLENGE);
    display_timer("[LVCS.Recompute] Interpolate", LVCS_RECOMPUTE_INTERPOLATE);
    display_timer("[LVCS.Recompute] Evaluate Polys", LVCS_RECOMPUTE_EVALUATE_POLYS);
    display_timer("[LVCS.Recompute] Recompute Evals", LVCS_RECOMPUTE_RECOMPUTE_EVALS);
    printf(" - PCS\n");
    display_timer("[PCS.Commit] Build Rows", PCS_COMMIT_BUILD_ROWS);
    display_timer("[PCS.Commit] Stack Rows", PCS_COMMIT_STACK_ROWS);
    display_timer("[PCS.Open] Build Coeffs", PCS_OPEN_BUILD_COEFFS);
    display_timer("[PCS.Open] Compute Evals", PCS_OPEN_COMPUTE_EVALS);
    display_timer("[PCS.Recompute] Build Coeffs", PCS_RECOMPUTE_BUILD_COEFFS);
    display_timer("[PCS.Recompute] Recompute Combis", PCS_RECOMPUTE_RECOMPUTE_COMBIS);
    printf(" - PIOP\n");
    display_timer("[PIOP.Run] XOF FPP", PIOP_RUN_XOF_FPP);
    display_timer("[PIOP.Run] Compute Individual Pol Constr.", PIOP_RUN_POL_CONSTRAINT_IND);
    display_timer("[PIOP.Run] Compute Individual Lin Constr.", PIOP_RUN_LIN_CONSTRAINT_IND);
    display_timer("[PIOP.Run] Compute Batched Pol Constr.", PIOP_RUN_POL_CONSTRAINT_OUT);
    display_timer("[PIOP.Run] Compute Batched Lin Constr.", PIOP_RUN_LIN_CONSTRAINT_OUT);
    display_timer("[PIOP.Recompute] XOF FPP", PIOP_RECOMPUTE_XOF_FPP);
    display_timer("[PIOP.Recompute] Compute Ind. Evals Pol Constr.", PIOP_RECOMPUTE_POL_CONSTRAINT_IND_EVALS);
    display_timer("[PIOP.Recompute] Compute Ind. Evals Lin Constr.", PIOP_RECOMPUTE_LIN_CONSTRAINT_IND_EVALS);
    display_timer("[PIOP.Recompute] Recompute Batched Pol Constr.", PIOP_RECOMPUTE_POL_CONSTRAINT_OUT);
    display_timer("[PIOP.Recompute] Recompute Batched Lin Constr.", PIOP_RECOMPUTE_LIN_CONSTRAINT_OUT);
    printf(" - Others\n");
    display_timer("Pin A", B_PIN_A);
    display_timer("Pin B", B_PIN_B);
    display_timer("Pin C", B_PIN_C);
    display_timer("Pin D", B_PIN_D);
    #endif

    return 0;
}


