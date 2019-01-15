
#version 450

#extension GL_ARB_separate_shader_objects : enable

layout (push_constant) uniform Material {
	vec4 base_color_factor;
	vec3 emissive_factor;
	float metallic_factor;
} material;

layout (location = 0) out vec4 outColor;

void main() {

    outColor =material.base_color_factor;
}
