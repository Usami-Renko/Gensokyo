
#version 450

layout (location = 0) in vec3 inPos;
layout (location = 1) in vec3 inNormal;

#define lightCount 6

layout (set = 0, binding = 0) uniform UBO {
	mat4 projection;
	mat4 view;
	mat4 model;
	mat4 y_correction;
} ubo;

layout (set = 0, binding = 1) uniform DynNode {
	mat4 transform;
} dyn_node;

layout(push_constant) uniform PushConsts {
	vec4 lightPos[lightCount];
} pushConsts;

layout (location = 0) out vec3 outNormal;
layout (location = 1) out vec4 outLightVec[lightCount];

out gl_PerVertex {
	vec4 gl_Position;
};

void main() {

	outNormal = inNormal;

	vec4 world_pos = ubo.model * dyn_node.transform * vec4(inPos.xyz, 1.0);

	gl_Position = ubo.y_correction * ubo.projection * ubo.view * world_pos;

	for (int i = 0; i < lightCount; ++i) {
		outLightVec[i].xyz = pushConsts.lightPos[i].xyz - inPos.xyz;
		// Store light radius in w
		outLightVec[i].w = pushConsts.lightPos[i].w;
	}
}
