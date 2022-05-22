#version 460

layout (location = 0) in vec2 position;

layout (set = 0, binding = 0) uniform GlobalUniforms {
    vec2 window_size;
};

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
