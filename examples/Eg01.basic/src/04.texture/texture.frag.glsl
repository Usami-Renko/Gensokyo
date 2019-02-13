
#version 450

#extension GL_ARB_separate_shader_objects : enable

// uniforms
layout (binding = 0) uniform sampler2D texSampler;

// fragment input
layout (location = 0) in vec2 fragTexCoord;

// fragment output
layout (location = 0) out vec4 outColor;

void main() {

    outColor = texture(texSampler, fragTexCoord);
}
