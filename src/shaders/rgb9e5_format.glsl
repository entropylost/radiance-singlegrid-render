#define RGB9E5_MANTISSA_BITS 9
#define RGB9E5_MANTISSA_MASK 0x1FF
#define RGB9E5_EXPONENT_BITS 5
#define RGB9E5_EXP_BIAS 15
#define RGB9E5_EXP_MAX ((1 << RGB9E5_EXPONENT_BITS) - RGB9E5_EXP_BIAS - 1)

#define MAX_RGB9E5_MANTISSA_VALUES (1 << RGB9E5_MANTISSA_BITS)
#define MAX_RGB9E5_MANTISSA (MAX_RGB9E5_MANTISSA_VALUES - 1)
#define MAX_RGB9E5 float(MAX_RGB9E5_MANTISSA) / float(MAX_RGB9E5_MANTISSA_VALUES) * exp2(float(RGB9E5_EXP_MAX))


uint to_rgb9e5(vec3 v) {
    vec3 clamped = clamp(v, vec3(0.0), vec3(MAX_RGB9E5));

    float max_val = max(max(clamped.r, clamped.g), clamped.b);
    
    int exponent =
        clamp(int(bitfieldExtract(floatBitsToUint(max_val), 23, 8)) // Discard sign
        - 126,  // Convert to signed, so exponent change 0 is actually at 0. 
        -RGB9E5_EXP_BIAS, RGB9E5_EXP_MAX + 1); // Clamp is inclusive
    
    uint written_exponent = uint(exponent + RGB9E5_EXP_BIAS);

    float scale = exp2(RGB9E5_MANTISSA_BITS - exponent);
    
    uvec3 mantissas = uvec3(clamped * scale + vec3(0.5));

    uint encoded = 
        (mantissas.x & RGB9E5_MANTISSA_MASK) |
        ((mantissas.y & RGB9E5_MANTISSA_MASK) << 9) |
        ((mantissas.z & RGB9E5_MANTISSA_MASK) << 18) |
        (written_exponent << 27);

    return encoded;
}

vec3 from_rgb9e5(uint encoded) {
    int exponent = int(encoded >> 27) - RGB9E5_EXP_BIAS;
    float scale = exp2(float(exponent - RGB9E5_MANTISSA_BITS));

    vec3 v = vec3(
        float(encoded & RGB9E5_MANTISSA_MASK),
        float((encoded >> 9) & RGB9E5_MANTISSA_MASK),
        float((encoded >> 18) & RGB9E5_MANTISSA_MASK)
    );

    return v * scale;
}
