#version 460
#extension GL_EXT_samplerless_texture_functions : enable
#include "rgb9e5_format.glsl"

layout (location = 0) out vec4 f_total_radiance;

layout (set = 0, binding = 0) uniform GlobalUniforms {
    vec2 window_size;
};

layout (set = 1, binding = 0) uniform RadianceCollectUniforms {
    uint light_directions;
};

layout (set = 2, binding = 0) uniform texture2D t_albedo;
layout (set = 2, binding = 1) uniform texture2D t_radiance;
layout (set = 2, binding = 2) uniform texture2D t_normal;

layout (set = 3, binding = 0) uniform utexture2DArray t_directional_radiance;


void main() {
    vec3 radiance = vec3(0);
    for (uint i = 0; i < light_directions; i++) {
        radiance += from_rgb9e5(texelFetch(t_directional_radiance, ivec3(gl_FragCoord.xy, i), 0).x);
    }
    vec4 albedo = texelFetch(t_albedo, ivec2(gl_FragCoord.xy), 0);
    f_total_radiance = vec4(radiance * albedo.xyz + texelFetch(t_radiance, ivec2(gl_FragCoord.xy), 0).xyz, albedo.w);
}
