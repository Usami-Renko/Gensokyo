
#version 450

#extension GL_ARB_separate_shader_objects: enable

layout (binding = 0, set = 0) uniform UboOjbect {
    mat4 translate;
    mat4 scale;
    mat4 rotate;
} ubo;

layout (location = 0) in vec4 inPosition;
layout (location = 1) in vec4 inColor;

layout (location = 0) out vec4 fragColor;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {

    gl_Position = ubo.translate * ubo.rotate * ubo.scale * inPosition;
    fragColor = inColor;
}
