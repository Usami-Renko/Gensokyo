
#version 450

#extension GL_ARB_separate_shader_objects: enable

layout (binding = 0, set = 0) uniform UboOjbect {
    mat4 rotate;
} ubo;

layout (location = 0) in vec2 inPosition;
layout (location = 1) in vec4 inColor;

layout (location = 0) out vec4 fragColor;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {

    gl_Position = ubo.rotate * vec4(inPosition, 0.0, 1.0);
    gl_Position.y = -gl_Position.y; // fix upside-down.

    fragColor = inColor;
}
