#version 450 core

layout(location = 0) in vec3 fColor;

layout(location = 0) out vec4 outColor;

void main(void) {
    outColor = vec4(fColor, 1.0);
}