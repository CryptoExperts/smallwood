#ifndef __CAPSS_SIGN_H__
#define __CAPSS_SIGN_H__

#include <stdint.h>

/**
 * @file capss-sign.h
 * @brief API for Signatures CAPSS
 * @ingroup capss_sign
 */

/**
 * \defgroup capss_sign Signatures CAPSS
 * \brief Signature Schemes built from the CAPSS framework
 */

/**
 * \brief Get the PK size, SK size and signature size
 * \ingroup capss_sign
 * \param variant the variant selector
 * \param pk_bytesize the public key size
 * \param sk_bytesize the secret key size
 * \param sig_max_bytesize an upper bound of the signature size
 */
int capss_sign_get_sizes(uint32_t variant, unsigned long long *pk_bytesize, unsigned long long *sk_bytesize, unsigned long long *sig_max_bytesize);

/**
 * \brief Generate public and private key.
 * \ingroup capss_sign
 * \param variant the variant selector
 * \param pk pointer to output public key
 * \param sk pointer to output secret key
 *
 * The pointers \p pk and \p sk should point to arrays respectively of \p pk_bytesize bytes and \p sk_bytesize bytes (outputs of \ref capss_sign_get_sizes).
 */
int capss_sign_keypair(uint32_t variant, unsigned char* pk, unsigned char* sk);

/**
 * \brief Compute signature.
 * \ingroup capss_sign
 * \param variant the variant selector
 * \param sig pointer to output signature
 * \param siglen pointer to output length of signature
 * \param m pointer to message to be signed
 * \param mlen length of message
 * \param sk pointer to the secret key
 *
 * The pointers \p sig and \p sk should point to arrays respectively of \p sig_max_bytesize bytes and \p sk_bytesize bytes (outputs of \ref capss_sign_get_sizes).
 */
int capss_sign_signature(uint32_t variant, unsigned char *sig, unsigned long long *siglen, const unsigned char *m,
                          unsigned long long mlen, const unsigned char *sk);

/**
 * \brief Compute signed message.
 * \ingroup capss_sign
 * \param variant the variant selector
 * \param sm pointer to output signed message
 * \param smlen pointer to output length of signed message
 * \param m pointer to message to be signed
 * \param mlen length of message
 * \param sk pointer to the secret key
 *
 * The pointers \p sm and \p sk should point to arrays respectively of " \p pk_bytesize + \p mlen" bytes and \p sk_bytesize bytes (outputs of \ref capss_sign_get_sizes).
 */
int capss_sign(uint32_t variant, unsigned char* sm, unsigned long long* smlen, const unsigned char* m,
                unsigned long long mlen, const unsigned char* sk);

/**
 * \brief Verify signature.
 * \ingroup capss_sign
 * \param variant the variant selector
 * \param sig pointer to input signature
 * \param siglen length of signature
 * \param m pointer to message
 * \param mlen length of message
 * \param pk pointer to the public key
 */
int capss_sign_verify(uint32_t variant, const unsigned char *sig, unsigned long long siglen, const unsigned char *m,
                       unsigned long long mlen, const unsigned char *pk);

/**
 * \brief Verify signed message.
 * \ingroup capss_sign
 * \param variant the variant selector
 * \param m pointer to output message
 * \param mlen pointer to output length of message
 * \param sm pointer to signed message
 * \param smlen length of signed message
 * \param pk pointer to the public key
 */
int capss_sign_open(uint32_t variant, unsigned char* m, unsigned long long* mlen, const unsigned char* sm,
                     unsigned long long smlen, const unsigned char* pk);

#ifdef CAPSS_VARIANT
/**
 * \brief Get the PK size, SK size and signature size
 * \ingroup capss_sign
 * \param pk_bytesize the public key size
 * \param sk_bytesize the secret key size
 * \param sig_max_bytesize an upper bound of the signature size
 * 
 * This function is available is available only if the code source is compile with a macro `CAPSS_VARIANT` which will be used to select the default variant.
 */
static inline int crypto_sign_get_sizes(unsigned long long *pk_bytesize, unsigned long long *sk_bytesize, unsigned long long *sig_max_bytesize) {
    return capss_sign_get_sizes(CAPSS_VARIANT, pk_bytesize, sk_bytesize, sig_max_bytesize);
}

/**
 * \brief Generate public and private key.
 * \ingroup capss_sign
 * \param pk pointer to output public key
 * \param sk pointer to output secret key
 *
 * The pointers \p pk and \p sk should point to arrays respectively of \p pk_bytesize bytes and \p sk_bytesize bytes (outputs of \ref crypto_sign_get_sizes). This function is available is available only if the code source is compile with a macro `CAPSS_VARIANT` which will be used to select the default variant.
 */
static inline int crypto_sign_keypair(unsigned char* pk, unsigned char* sk) {
    return capss_sign_keypair(CAPSS_VARIANT, pk, sk);
}

/**
 * \brief Compute signature.
 * \ingroup capss_sign
 * \param sig pointer to output signature
 * \param siglen pointer to output length of signature
 * \param m pointer to message to be signed
 * \param mlen length of message
 * \param sk pointer to the secret key
 *
 * The pointers \p sig and \p sk should point to arrays respectively of \p sig_max_bytesize bytes and \p sk_bytesize bytes (outputs of \ref crypto_sign_get_sizes). This function is available is available only if the code source is compile with a macro `CAPSS_VARIANT` which will be used to select the default variant.
 */
static inline int crypto_sign_signature(unsigned char *sig, unsigned long long *siglen, const unsigned char *m,
                          unsigned long long mlen, const unsigned char *sk) {
    return capss_sign_signature(CAPSS_VARIANT, sig, siglen, m, mlen, sk);
}

/**
 * \brief Compute signed message.
 * \ingroup capss_sign
 * \param sm pointer to output signed message
 * \param smlen pointer to output length of signed message
 * \param m pointer to message to be signed
 * \param mlen length of message
 * \param sk pointer to the secret key
 *
 * The pointers \p sm and \p sk should point to arrays respectively of " \p pk_bytesize + \p mlen" bytes and \p sk_bytesize bytes (outputs of \ref crypto_sign_get_sizes). This function is available is available only if the code source is compile with a macro `CAPSS_VARIANT` which will be used to select the default variant.
 */
static inline int crypto_sign(unsigned char* sm, unsigned long long* smlen, const unsigned char* m,
                unsigned long long mlen, const unsigned char* sk) {
    return capss_sign(CAPSS_VARIANT, sm, smlen, m, mlen, sk);
}

/**
 * \brief Verify signature.
 * \ingroup capss_sign
 * \param sig pointer to input signature
 * \param siglen length of signature
 * \param m pointer to message
 * \param mlen length of message
 * \param pk pointer to the public key
 * 
 * This function is available is available only if the code source is compile with a macro `CAPSS_VARIANT` which will be used to select the default variant.
 */
static inline int crypto_sign_verify(const unsigned char *sig, unsigned long long siglen, const unsigned char *m,
                       unsigned long long mlen, const unsigned char *pk) {
    return capss_sign_verify(CAPSS_VARIANT, sig, siglen, m, mlen, pk);
}

/**
 * \brief Verify signed message.
 * \ingroup capss_sign
 * \param m pointer to output message
 * \param mlen pointer to output length of message
 * \param sm pointer to signed message
 * \param smlen length of signed message
 * \param pk pointer to the public key
 * 
 * This function is available is available only if the code source is compile with a macro `CAPSS_VARIANT` which will be used to select the default variant.
 */
static inline int crypto_sign_open(unsigned char* m, unsigned long long* mlen, const unsigned char* sm,
                     unsigned long long smlen, const unsigned char* pk) {
    return capss_sign_open(CAPSS_VARIANT, m, mlen, sm, smlen, pk);
}
#endif

#endif /* __CAPSS_SIGN_H__ */
