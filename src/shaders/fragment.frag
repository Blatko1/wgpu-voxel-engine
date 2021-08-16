#version 450 core

layout(location = 0) in vec3 o_pos;

layout(location = 0) out vec4 outColor;

void main(void) {
    outColor = vec4(o_pos, 1.0);
}