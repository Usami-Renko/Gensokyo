
#version 450

layout (location = 0) in vec3 inPos;
layout (location = 1) in vec3 inNormal;
layout (location = 2) in vec2 inUV;

layout (location = 0) out vec2 outUV;
layout (location = 1) out float outLodBias;
layout (location = 2) flat out int outSamplerIndex;
layout (location = 3) out vec3 outNormal;
layout (location = 4) out vec3 outViewVec;
layout (location = 5) out vec3 outLightVec;

layout (set = 0, binding = 0) uniform UBO {
	mat4 projection;
	mat4 view;
	mat4 model;
	mat4 ycorrection;
	vec4 viewPos;
	float lodBias;
	int samplerIndex;
} ubo;

layout (set = 0, binding = 1) uniform DynNode {
	mat4 transform;
} dyn_node;

out gl_PerVertex {
	vec4 gl_Position;
};

void main() {

	outUV = inUV * vec2(2.0, 1.0);
	outLodBias = ubo.lodBias;
	outSamplerIndex = ubo.samplerIndex;

	mat4 world_pos = ubo.model * dyn_node.transform;

	vec3 worldPos = vec3(world_pos * vec4(inPos, 1.0));

	gl_Position = ubo.ycorrection * ubo.projection * ubo.view * world_pos * vec4(inPos.xyz, 1.0);

	outNormal = mat3(inverse(transpose(world_pos))) * inNormal;
	vec3 lightPos = vec3(-30.0, 0.0, 0.0);
	outLightVec = worldPos - lightPos;
	outViewVec = ubo.viewPos.xyz - worldPos;
}
