
#version 450

#extension GL_ARB_separate_shader_objects: enable

layout (binding = 0, set = 0) uniform UboOjbect {
    mat4 projection;
    mat4 view;
    mat4 model;
} ubo;

layout (location = 0) in vec4 inPosition;
layout (location = 1) in vec2 inTexCoord;

layout (location = 0) out vec2 fragTexCoord;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {

    vec4 model = ubo.model * inPosition;
    gl_Position = ubo.projection * ubo.view * model;
    fragTexCoord = inTexCoord;
}
