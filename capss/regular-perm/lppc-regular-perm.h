#ifndef __LPPC_REGULAR_PERM_H__
#define __LPPC_REGULAR_PERM_H__

#include "lppc.h"

/**
 * \struct reg_perm_lppc_cfg_t
 * \brief Configuration of a Regular-Permutation LPPC statement
 * \headerfile ""
 */
typedef struct reg_perm_lppc_cfg_t reg_perm_lppc_cfg_t;

/**
 * \struct reg_perm_lppc_t
 * \ingroup proof
 * \brief Instance of a Regular-Permutation LPPC statement
 * \headerfile ""
 */
typedef struct reg_perm_lppc_t reg_perm_lppc_t;

/**
 * \brief Allocate a configuration of a Regular-Permutation LPPC statement
 * \relates reg_perm_lppc_cfg_t
 * \param batching_factor the batching factor
 * \param iv_size the IV size
 * \param y_size the size of the public permutation output
 * \return the corresponding configuration
 */
reg_perm_lppc_cfg_t* rperm_lppc_malloc_config(uint32_t batching_factor, uint32_t iv_size, uint32_t y_size);

/**
 * \brief Format a configuration of a Regular-Permutation LPPC statement as a configuration of a generic LPPC statement
 * \relates reg_perm_lppc_cfg_t
 * \param lppc_cfg a configuration of a Regular-Permutation LPPC statement
 * \return a configuration of a generic LPPC statement
 */
const lppc_cfg_t* rperm_lppc_get_config_lppc(const reg_perm_lppc_cfg_t* lppc_cfg);

/**
 * \brief Format a Regular-Permutation LPPC statement as a generic LPPC statement
 * \relates reg_perm_lppc_t
 * \param lppc a Regular-Permutation LPPC statement
 * \return a generic LPPC statement
 */
const reg_perm_lppc_cfg_t* rperm_lppc_get_config(const reg_perm_lppc_t* lppc);

/**
 * \brief Allocate a Regular-Permutation LPPC statement from a configuration, a IV and a public permutation output
 * \relates reg_perm_lppc_t
 * \param lppc_cfg a configuration of a Regular-Permutation LPPC statement
 * \param iv a initialization vector
 * \param y a public permutation output
 * \return a Regular-Permutation LPPC statement
 */
reg_perm_lppc_t* rperm_lppc_malloc(const reg_perm_lppc_cfg_t* lppc_cfg, const vec_t iv, const vec_t y);

/**
 * \brief Sample a Regular-Permutation LPPC statement with a witness from a configuration
 * \relates reg_perm_lppc_t
 * \param lppc_cfg a configuration of a Regular-Permutation LPPC statement
 * \param lppc the sampled Regular-Permutation LPPC statement
 * \param secret the corresponding secret vector
 * \return 0 if the sampling is successful, a non-zero value otherwise
 * 
 * The witness vector can be built from the secret vector using \ref rperm_lppc_build_witness.
 */
int rperm_lppc_random(const reg_perm_lppc_cfg_t* lppc_cfg, reg_perm_lppc_t** lppc, vec_t* secret);

/**
 * \brief Get the size (in field elements) of the secret vector
 * \relates reg_perm_lppc_cfg_t
 * \param lppc_cfg a configuration of a Regular-Permutation LPPC statement
 * \return a size in field elements
 */
uint32_t rperm_lppc_get_secret_size(const reg_perm_lppc_cfg_t* lppc_cfg);

/**
 * \brief Build the witness vector from the secret vector
 * \relates reg_perm_lppc_t
 * \param lppc a Regular-Permutation LPPC statement
 * \param secret the secret vector
 * \param witness the built witness vector
 * \return 0 if successful, a non-zero value otherwise
 */
int rperm_lppc_build_witness(const reg_perm_lppc_t* lppc, const vec_t secret, vec_t witness);

/**
 * \brief Return the byte size of a serialized Regular-Permutation LPPC statement
 * \relates reg_perm_lppc_cfg_t
 * \param lppc_cfg a configuration of a Regular-Permutation LPPC statement
 * \return a byte size
 */
uint32_t rperm_lppc_get_serialized_bytesize(const reg_perm_lppc_cfg_t* lppc_cfg);

/**
 * \brief Serialize a Regular-Permutation LPPC statement
 * \relates reg_perm_lppc_t
 * \param buffer a byte array that contains the serialized LPPC statement
 * \param lppc the LPPC statement to serialize
 * 
 * The byte size of the buffer should be (at least) of the size given by \ref rperm_lppc_get_serialized_bytesize.
 */
void rperm_lppc_serialize(uint8_t* buffer, const reg_perm_lppc_t* lppc);

/**
 * \brief Deserialize a Regular-Permutation LPPC statement
 * \relates reg_perm_lppc_t
 * \param lppc_cfg a configuration of a Regular-Permutation LPPC statement
 * \param buffer a byte array that contains the LPPC statement
 * \return the deserialized LPPC statement
 */
reg_perm_lppc_t* rperm_lppc_malloc_from_serialized(const reg_perm_lppc_cfg_t* lppc_cfg, const uint8_t* buffer);

#endif /* __LPPC_REGULAR_PERM_H__ */
