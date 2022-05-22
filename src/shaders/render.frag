#version 460
#extension GL_EXT_samplerless_texture_functions : enable

layout (location = 0) out vec4 f_color;

layout (set = 0, binding = 0) uniform GlobalUniforms {
    vec2 window_size;
};

layout (set = 1, binding = 0) uniform texture2D t_total_radiance;

vec4 into_srgb(vec4 linear) {
    return vec4(pow(linear.xyz, vec3(1.0 / 2.2)), linear.w);
}

void main() {
    f_color = into_srgb(texelFetch(t_total_radiance, ivec2(gl_FragCoord.xy), 0));
}
