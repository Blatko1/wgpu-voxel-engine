#version 450 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 color;
layout(location = 2) in vec2 texCoords;

layout(set = 0, binding = 0) uniform GlobalMatrix {
    mat4 matrix;
};

layout(location = 0) out vec3 fColor;
layout(location = 1) out vec2 fTexCoords;

void main(void) {
    gl_Position = matrix * vec4(pos, 1.0);
    fColor = color;
    fTexCoords = texCoords;
}