#version 450 core

layout(location = 0) in vec3 pos;

layout(set = 0, binding = 0) uniform GlobalMatrix {
    mat4 matrix;
};

void main(void) {
    gl_Position = matrix * vec4(pos, 1.0);
}