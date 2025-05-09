#ifndef __FIELD_BASE_STRUCT_H__
#define __FIELD_BASE_STRUCT_H__

#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#define FIELD_BYTESIZE 32
typedef uint8_t felt_t[FIELD_BYTESIZE];

extern int randombytes(unsigned char* x, unsigned long long xlen);

extern void bn254_field_reduce(uint8_t* c, const uint8_t* a);
extern void bn254_field_add(uint8_t* c, const uint8_t* a, const uint8_t* b);
extern void bn254_field_neg(uint8_t* c, const uint8_t* a);
extern void bn254_field_sub(uint8_t* c, const uint8_t* a, const uint8_t* b);
extern void bn254_field_mul(uint8_t* c, const uint8_t* a, const uint8_t* b);
extern void bn254_field_inv(uint8_t* c, const uint8_t* a);
extern void bn254_field_div(uint8_t* c, const uint8_t* a, const uint8_t* b);
extern int bn254_field_is_equal(const uint8_t* a, const uint8_t* b);

static inline double felt_get_log2_field_order(void) {
    return 253.214834;
}

static inline uint32_t felt_get_bytesize(void) { return FIELD_BYTESIZE; }

static inline void felt_random(felt_t* a) {
    randombytes((uint8_t*) (*a), FIELD_BYTESIZE);
    bn254_field_reduce((uint8_t*) (*a), (uint8_t*) (*a));
}

static inline void felt_set(felt_t* c, const felt_t* a) {
    memcpy((*c), (*a), FIELD_BYTESIZE);
}

static inline void felt_from_uint32(felt_t* c, uint32_t a) {
    memset((*c), 0, FIELD_BYTESIZE);
    (*c)[0] = (a >>  0) & 0xff;
    (*c)[1] = (a >>  8) & 0xff;
    (*c)[2] = (a >> 16) & 0xff;
    (*c)[3] = (a >> 24) & 0xff;
}

static inline uint32_t felt_to_uint32(const felt_t* a) {
    uint32_t c = 0;
    c |= ((uint32_t) (*a)[0]) <<  0;
    c |= ((uint32_t) (*a)[1]) <<  8;
    c |= ((uint32_t) (*a)[2]) << 16;
    c |= ((uint32_t) (*a)[3]) << 24;
    return c;
}

static inline void felt_from_bytestring(felt_t* c, const uint8_t* a) {
    memcpy((*c), a, FIELD_BYTESIZE);
    bn254_field_reduce((uint8_t*) (*c), (uint8_t*) (*c));
}

static inline void felt_printf(const felt_t* a) {
	for(uint32_t idx = 0; idx < FIELD_BYTESIZE; idx++)
		printf("%02x", (*a)[idx]);
}

static inline void felt_set_zero(felt_t* c) {
    memset((*c), 0, FIELD_BYTESIZE);
}

static inline void felt_set_one(felt_t* c) {
    memset((*c), 0, FIELD_BYTESIZE);
    (*c)[0] = 1;
}

static inline void felt_add(felt_t* c, const felt_t* a, const felt_t* b) {
    bn254_field_add((uint8_t*) (*c), (const uint8_t*) (*a), (const uint8_t*) (*b));
}

static inline void felt_neg(felt_t* c, const felt_t* a) {
    bn254_field_neg((uint8_t*) (*c), (const uint8_t*) (*a));
}

static inline void felt_sub(felt_t* c, const felt_t* a, const felt_t* b) {
    bn254_field_sub((uint8_t*) (*c), (const uint8_t*) (*a), (const uint8_t*) (*b));
}

static inline void felt_mul(felt_t* c, const felt_t* a, const felt_t* b) {
    bn254_field_mul((uint8_t*) (*c), (const uint8_t*) (*a), (const uint8_t*) (*b));
}

static inline void felt_mul_add(felt_t* c, const felt_t* a, const felt_t* b) {
    felt_t tmp;
    bn254_field_mul((uint8_t*) tmp, (const uint8_t*) (*a), (const uint8_t*) (*b));
    bn254_field_add((uint8_t*) (*c), (uint8_t*) (*c), (uint8_t*) tmp);
}

static inline void felt_inv(felt_t* c, const felt_t* a) {
    bn254_field_inv((uint8_t*) (*c), (const uint8_t*) (*a));
}

static inline void felt_div(felt_t* c, const felt_t* a, const felt_t* b) {
    bn254_field_div((uint8_t*) (*c), (const uint8_t*) (*a), (const uint8_t*) (*b));
}

static inline int felt_is_equal(const felt_t* a, const felt_t* b) {
    return bn254_field_is_equal((const uint8_t*) (*a), (const uint8_t*) (*b));;
}

static inline int felt_is_zero(const felt_t* a) {
    felt_t zero;
    felt_set_zero(&zero);
    return felt_is_equal(a, &zero);
}

static inline void felt_serialize(uint8_t* buffer, const felt_t* a) {
    memcpy(buffer, a, FIELD_BYTESIZE);
}

static inline void felt_deserialize(felt_t* a, const uint8_t* buffer) {
    memcpy(a, buffer, FIELD_BYTESIZE);
}


#endif /* __FIELD_BASE_STRUCT_H__ */

