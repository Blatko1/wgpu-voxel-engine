#version 450 core

layout(location = 0) in vec3 pos;

layout(location = 0) out vec3 o_pos;

void main(void) {
    gl_Position = vec4(pos, 1.0);
    o_pos = pos;
}