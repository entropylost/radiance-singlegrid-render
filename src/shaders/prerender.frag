#version 460

layout (location = 0) in vec4 o_albedo_lin;
layout (location = 1) in vec4 o_radiance_lin;
layout (location = 2) in vec2 o_normal;
layout (location = 0) out vec4 f_albedo_lin;
layout (location = 1) out vec4 f_radiance_lin;
layout (location = 2) out vec2 f_normal;

layout (set = 0, binding = 0) uniform GlobalUniforms {
    vec2 window_size;
};

void main() {
    f_albedo_lin = o_albedo_lin;
    f_radiance_lin = o_radiance_lin;
    f_normal = o_normal;
}
