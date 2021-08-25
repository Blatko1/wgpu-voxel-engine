#version 450 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 texCoords;

// Instance Matrix
layout(location = 3) in vec4 instanceMatrix1;
layout(location = 4) in vec4 instanceMatrix2;
layout(location = 5) in vec4 instanceMatrix3;
layout(location = 6) in vec4 instanceMatrix4;

// Texture index
layout(location = 7) in uint textureIndex;

layout(set = 0, binding = 0) uniform GlobalMatrix {
    mat4 matrix;
};

layout(location = 0) out vec2 fTexCoords;
layout(location = 1) flat out uint fTextureIndex;

void main(void) {
    mat4 instance = mat4(instanceMatrix1, instanceMatrix2, instanceMatrix3, instanceMatrix4);
    gl_Position = matrix * instance * vec4(pos, 1.0);
    fTexCoords = texCoords;
    fTextureIndex = textureIndex;
}