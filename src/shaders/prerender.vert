#version 460

layout (location = 0) in vec2 position;
layout (location = 1) in vec2 normal;
layout (location = 2) in vec4 albedo_lin;
layout (location = 3) in vec4 radiance_lin;
layout (location = 0) out vec4 o_albedo_lin;
layout (location = 1) out vec4 o_radiance_lin;
layout (location = 2) out vec2 o_normal;

layout (set = 0, binding = 0) uniform GlobalUniforms {
    vec2 window_size;
};

void main() {
    gl_Position = vec4(position / window_size * 2.0 - vec2(1.0), 0.0, 1.0);
    o_albedo_lin = albedo_lin;
    o_radiance_lin = radiance_lin;
    o_normal = normal;
}
