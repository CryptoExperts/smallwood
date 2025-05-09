#include "capss-sign.h"
#include "lppc-regular-perm.h"
#include "smallwood.h"
#include "utils.h"
#include <stdio.h>
#include "benchmark.h"

static void serialize_uint32(uint8_t buf[4], uint32_t x) {
    buf[0] = x & 0xFF;
    buf[1] = (x >> 8) & 0xFF;
    buf[2] = (x >> 16) & 0xFF;
    buf[3] = x >> 24;
}

static uint32_t deserialize_uint32(const uint8_t buf[4]) {
    return ((uint32_t) buf[0]) | ((uint32_t) buf[1] << 8) | ((uint32_t) buf[2] << 16) | ((uint32_t) buf[3] << 24);
}

extern int get_config_lppc_smallwood(uint32_t variant, reg_perm_lppc_cfg_t** lppc_cfg, smallwood_t** sw);

int capss_sign_get_sizes(uint32_t variant, unsigned long long *pk_bytesize, unsigned long long *sk_bytesize, unsigned long long *sig_max_bytesize) {
    reg_perm_lppc_cfg_t* lppc_cfg;
    smallwood_t* sw;
    int ret = get_config_lppc_smallwood(variant, &lppc_cfg, &sw);
    if(ret != 0)
        return -1;

    *sig_max_bytesize = smallwood_max_sizeof_proof(sw);
    *pk_bytesize = rperm_lppc_get_serialized_bytesize(lppc_cfg);
    *sk_bytesize = (*pk_bytesize) + vec_get_bytesize(rperm_lppc_get_secret_size(lppc_cfg));

    free(lppc_cfg);
    free(sw);
    return 0;
}

int capss_sign_keypair(uint32_t variant, unsigned char* pk, unsigned char* sk) {
    reg_perm_lppc_cfg_t* lppc_cfg;
    int ret = get_config_lppc_smallwood(variant, &lppc_cfg, NULL);
    if(ret != 0)
        return -1;

    vec_t secret;
    reg_perm_lppc_t* lppc;
    ret = rperm_lppc_random(lppc_cfg, &lppc, &secret);
    if(ret != 0) {
        free(lppc_cfg);
        return -1;
    }

    uint32_t pk_bytesize = rperm_lppc_get_serialized_bytesize(lppc_cfg);
    rperm_lppc_serialize(pk, lppc);
    WRITE_BUFFER_BYTES(sk, pk, pk_bytesize);
    vec_serialize(sk, secret, rperm_lppc_get_secret_size(lppc_cfg));
    free(secret);
    free(lppc);
    free(lppc_cfg);
    return 0;
}

int capss_sign_signature(uint32_t variant, unsigned char *sig, unsigned long long *siglen, const unsigned char *m,
    unsigned long long mlen, const unsigned char *sk) {

    reg_perm_lppc_cfg_t* lppc_cfg;
    smallwood_t* sw;
    int ret = get_config_lppc_smallwood(variant, &lppc_cfg, &sw);
    if(ret != 0)
        return -1;

    // Parse the secret key (and the public key)
    uint32_t pk_bytesize = rperm_lppc_get_serialized_bytesize(lppc_cfg);
    const uint8_t* pk = sk;
    sk += pk_bytesize;

    reg_perm_lppc_t* lppc = rperm_lppc_malloc_from_serialized(lppc_cfg, pk);
    vec_t secret = malloc_vec(rperm_lppc_get_secret_size(lppc_cfg));
    vec_t witness = malloc_vec(lppc_get_witness_size(rperm_lppc_get_config_lppc(lppc_cfg)));
    if(lppc == NULL || secret == NULL || witness == NULL) {
        free(lppc_cfg);
        free(lppc);
        free(secret);
        free(witness);
        return -1;
    }

    vec_deserialize(secret, sk, rperm_lppc_get_secret_size(lppc_cfg));
    ret = rperm_lppc_build_witness(lppc, secret, witness);
    free(secret);
    if(ret != 0) {
        free(lppc_cfg);
        free(lppc);
        free(witness);
        return -1;
    }

    // Build signature
    uint32_t proof_size;
    uint8_t* proof = smallwood_prove_with_data(sw, (lppc_t*) lppc, witness, m, mlen, &proof_size);
    free(sw);
    free(lppc_cfg);
    free(lppc);
    free(witness);
    if(proof == NULL)
        return -1;

    *siglen = proof_size;
    memcpy(sig, proof, proof_size);
    free(proof);

    return 0;
}

int capss_sign(uint32_t variant,
    unsigned char *sm, unsigned long long *smlen,
    const unsigned char *m, unsigned long long mlen,
    const unsigned char *sk) {

    uint32_t signature_len = 0;
    memmove(sm + sizeof(signature_len), m, mlen);
    int ret = capss_sign_signature(variant, sm + sizeof(signature_len) + mlen, smlen, sm + sizeof(signature_len), mlen, sk);
    if(ret == 0) {
        signature_len = (uint32_t) *smlen;
        *smlen += mlen + sizeof(uint32_t);
        serialize_uint32(sm, signature_len);    
    }
    return ret;
}

int capss_sign_verify(uint32_t variant, const unsigned char *sig, unsigned long long siglen, const unsigned char *m,
    unsigned long long mlen, const unsigned char *pk) {

    reg_perm_lppc_cfg_t* lppc_cfg;
    smallwood_t* sw;
    int ret = get_config_lppc_smallwood(variant, &lppc_cfg, &sw);
    if(ret != 0)
        return -1;

    // Parse the secret key (and the public key)
    reg_perm_lppc_t* lppc = rperm_lppc_malloc_from_serialized(lppc_cfg, pk);
    if(lppc == NULL) {
        free(lppc_cfg);
        free(lppc);
        return -1;
    }

    int verif = smallwood_verify_with_data(sw, (lppc_t*) lppc, m, mlen, sig, siglen);
    free(sw);
    free(lppc_cfg);
    free(lppc);
    return verif;
}

int capss_sign_open(uint32_t variant,
    unsigned char *m, unsigned long long *mlen,
    const unsigned char *sm, unsigned long long smlen,
    const unsigned char *pk) {

    uint32_t signature_len = 0;
    // The signature is too short to hold the signature length.
    if (smlen < sizeof(signature_len)) {
        return -1;
    }

    signature_len = deserialize_uint32(sm);
    // The signature is too short to hold the signature
    if (signature_len + sizeof(uint32_t) > smlen) {
        return -1;
    }

    const size_t message_len = smlen - signature_len - sizeof(uint32_t);
    const uint8_t* message   = sm + sizeof(uint32_t);
    const uint8_t* sig       = sm + sizeof(uint32_t) + message_len;

    int ret = capss_sign_verify(variant, sig, signature_len, message, message_len, pk);
    if (ret) {
        return ret;
    }

    memmove(m, message, message_len);
    *mlen = message_len;
    return 0;
}
