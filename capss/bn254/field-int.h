#ifndef __FIELD_INT_H__
#define __FIELD_INT_H__

#include "field-base.h"

static void felt_int_left_shift(felt_t* c, const felt_t* a, uint32_t bit_shift) {
    uint32_t byte_shift = bit_shift / 8;
    uint32_t bit_shift_mod = bit_shift % 8;

    for(uint32_t i = 0; i < FIELD_BYTESIZE; i++) {
        uint8_t value = 0;

        if((FIELD_BYTESIZE-1)-i >= byte_shift) {
            value |= (*a)[(FIELD_BYTESIZE-1)-i-byte_shift] << bit_shift_mod;
        }
        if(bit_shift_mod > 0 && (FIELD_BYTESIZE-1)-i >= byte_shift+1) {
            value |= (*a)[(FIELD_BYTESIZE-1)-i-(byte_shift+1)] >> (8-bit_shift_mod);
        }

        (*c)[(FIELD_BYTESIZE-1)-i] = value;
    }
}

static inline void felt_int_minus_one(felt_t* c, const felt_t* a) {
    uint8_t carry = 1;
    
    for (uint32_t i = 0; i < FIELD_BYTESIZE; i++) {
        uint8_t byte = (*a)[i];
        
        if (carry) {
            if (byte == 0) {
                (*c)[i] = 0xFF;
            } else {
                (*c)[i] = byte - 1;
                carry = 0;
            }
        } else {
            (*c)[i] = byte;
        }
    }

    if(carry) {
        // It assumes that the largest integer is represented by "-1"
        felt_t one;
        felt_set_zero(c);
        felt_set_one(&one);
        felt_sub(c, c, &one);
    }
}

static inline int felt_int_leq(const felt_t* a, const felt_t* b) {
    for (int i = FIELD_BYTESIZE - 1; i >= 0; i--) {
        if ((*a)[i] < (*b)[i]) return 1;
        if ((*a)[i] > (*b)[i]) return 0;
    }
    return 1;
}

#if FIELD_BYTESIZE % 4 != 0
#error FIELD_BYTESIZE % 4 != 0, for felt_div_euclid
#endif
static inline void felt_int_div_euclid(felt_t* q, uint32_t* r, const felt_t* a, uint32_t d) {
    uint64_t remainder = 0;
    uint32_t* dividend = (uint32_t*) a;
    uint32_t* quotient = (uint32_t*) q;
    uint32_t divisor = d;

    for (int i = (FIELD_BYTESIZE/4) - 1; i >= 0; i--) {
        remainder = (remainder << 32) | dividend[i];

        quotient[i] = (uint32_t)(remainder / divisor);
        remainder = remainder % divisor;
    }

    *r = (uint32_t)remainder;
}



#endif /* __FIELD_INT_H__ */

