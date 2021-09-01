#version 450 core

#extension GL_EXT_nonuniform_qualifier : require

layout(location = 0) in vec2 fTexCoords;
layout(location = 1) flat in uint fTextureIndex;

/*layout(set = 1, binding = 0) uniform sampler t_sample;
layout(set = 1, binding = 1) uniform texture2D textures[];*/

layout(location = 0) out vec4 outColor;

void main(void) {
    outColor = vec4(0.1, 0.1, 0.1, 1.0); /*texture(sampler2D(textures[fTextureIndex], t_sample), fTexCoords);*/
}