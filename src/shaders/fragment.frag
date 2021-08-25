#version 450 core

layout(location = 0) in vec3 fColor;
layout(location = 1) in vec2 fTexCoords;

layout(set = 1, binding = 0) uniform sampler t_sample;
layout(set = 1, binding = 1) uniform texture2D textures[];

layout(location = 0) out vec4 outColor;

void main(void) {
    outColor = texture(sampler2D(textures[0], t_sample), fTexCoords) * vec4(fColor, 1.0);
}