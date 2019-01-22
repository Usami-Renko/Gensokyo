
#version 450

#extension GL_ARB_separate_shader_objects: enable

layout (binding = 0, set = 0) uniform UboOjbect {
    mat4 projection;
    mat4 view;
    mat4 model;
    mat4 y_correction; // `y_correction` is used to fix y-coordinate upside-down.
} ubo;

layout (location = 0) in vec4 inPosition;
layout (location = 1) in vec4 inColor;

layout (location = 0) out vec4 fragColor;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {

    vec4 model = ubo.model * inPosition;
    gl_Position = ubo.y_correction * ubo.projection * ubo.view * model;

    fragColor = inColor;
}
