
#version 450

#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 1) uniform PBRMaterial {
    vec4 base_color_factor;
    float metallic_factor;
} pbr_mat;

layout (location = 0) out vec4 outColor;

void main() {

    outColor = pbr_mat.base_color_factor;
}
