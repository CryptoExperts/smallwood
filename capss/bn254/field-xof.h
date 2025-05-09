#ifndef __FIELD_XOF_H__
#define __FIELD_XOF_H__

extern void bn254_xof(uint8_t* output, const uint8_t* input, uint32_t input_felt_size, uint32_t output_felt_size);
extern void bn254_compress2(uint8_t* output, const uint8_t* input);
extern void bn254_compress4(uint8_t* output, const uint8_t* input);

static inline void vec_xof(vec_t out, const vec_t in, uint32_t in_size, uint32_t out_size) {
    if(in_size == 4 && out_size == 1) {
        bn254_compress4((uint8_t*) out, (const uint8_t*) in);
    } else if(in_size == 3 && out_size == 1) {
        uint8_t input[FIELD_BYTESIZE*4];
        memcpy(input, in, 3*FIELD_BYTESIZE);
        memset(input+3*FIELD_BYTESIZE, 0, FIELD_BYTESIZE);
        bn254_compress4((uint8_t*) out, (const uint8_t*) input);
    } else if(in_size == 2 && out_size == 1) {
        bn254_compress2((uint8_t*) out, (const uint8_t*) in);
    } else if(in_size == 1 && out_size == 1) {
        uint8_t input[FIELD_BYTESIZE*2];
        memcpy(input, in, FIELD_BYTESIZE);
        memset(input+FIELD_BYTESIZE, 0, FIELD_BYTESIZE);
        bn254_compress2((uint8_t*) out, (const uint8_t*) input);
    } else {
        bn254_xof((uint8_t*) out, (const uint8_t*) in, in_size, out_size);
    }
}

#endif /* __FIELD_XOF_H__ */

