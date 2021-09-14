#version 450 core

layout(location = 0) in vec2 fTexCoords;
layout(location = 1) flat in uint fTextureIndex;

layout(set = 1, binding = 0) uniform sampler t_sample;
layout(set = 1, binding = 1) uniform texture2D textures[4];

layout(location = 0) out vec4 outColor;

void main(void) {
    outColor = texture(sampler2D(textures[fTextureIndex], t_sample), fTexCoords);
}